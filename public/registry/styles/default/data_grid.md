---
title: "Data Grid"
name: "data_grid"
cargo_dependencies: []
registry_dependencies: ["checkbox", "dropdown_menu"]
type: "components:ui"
path: "ui/data_grid.rs"
---

# Data Grid

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add data_grid
```

## Component Code

```rust
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use icons::{ArrowDownWideNarrow, ArrowUpNarrowWide, ChevronDown, CircleX, EyeOff, PanelLeft, PanelLeftClose};
use leptos::ev::KeyboardEvent;
use leptos::html;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::hooks::use_cell_edit::CellEditContext;
use crate::components::hooks::use_virtual_scroll::{VirtualScrollState, use_virtual_scroll};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuRadioGroup,
    DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
};

/// Enforces display logic (class + value) to live in data structs, not inline in views.
#[derive(Debug, Clone, Default)]
pub struct StyledGridCell {
    pub class: &'static str,
    pub value: String,
}

impl StyledGridCell {
    pub fn new(class: &'static str, value: String) -> Self {
        Self { class, value }
    }
}

// * Source: https://tablecn.com/data-grid

/* ========================================================== */
/*                     ✨ TYPES ✨                            */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    None,
    Asc,
    Desc,
}

/* ========================================================== */
/*                     ✨ TRAITS ✨                           */
/* ========================================================== */

/// Trait for columns that can be pinned in a data grid.
/// Implement this trait on your Column enum to use the pinning utility functions.
pub trait PinnableColumn: Eq + Hash + Copy {
    /// Returns the list of pinnable columns with their widths in display order.
    fn pinnable_columns() -> &'static [(Self, i32)];
}

/// Trait for columns that support sorting.
/// Generic over the row type `R` so each data table can define its own row structure.
///
/// # Example
/// ```ignore
/// impl SortableColumn<RowData> for Column {
///     fn compare(self, a: &RowData, b: &RowData) -> Option<std::cmp::Ordering> {
///         match self {
///             Self::Name => Some(a.name.cmp(&b.name)),
///             Self::Age => Some(a.age.cmp(&b.age)),
///             _ => None,
///         }
///     }
/// }
/// ```
pub trait SortableColumn<R: Default>: Copy {
    /// Compares two row items by this column's field.
    /// Returns None for columns that don't support sorting.
    fn compare(self, a: &R, b: &R) -> Option<std::cmp::Ordering>;

    /// Sorts rows in place by this column in the given direction.
    /// Default implementation uses `compare`.
    fn sort_rows(self, rows: &mut [R], direction: SortDirection) {
        if direction == SortDirection::None {
            return;
        }
        // Check if this column supports sorting
        if self.compare(&R::default(), &R::default()).is_some() {
            match direction {
                SortDirection::Asc => rows.sort_by(|a, b| self.compare(a, b).unwrap_or(std::cmp::Ordering::Equal)),
                SortDirection::Desc => {
                    rows.sort_by(|a, b| self.compare(a, b).unwrap_or(std::cmp::Ordering::Equal).reverse())
                }
                SortDirection::None => {}
            }
        }
    }
}

/// Trait for row data that can be displayed in a generic data grid.
/// Implement this trait on your data struct to use with `GenericDataGrid`.
///
/// # Example
/// ```ignore
/// impl DataGridRow for LinkedinCreator {
///     type Id = i32;
///     type Column = Column;
///
///     fn id(&self) -> Self::Id { self.id }
///     fn matches_filter(&self, filter: &str) -> bool {
///         self.name.to_lowercase().contains(filter)
///     }
///     fn get_value(&self, col: Self::Column) -> String {
///         match col {
///             Column::Name => self.name.clone(),
///             // ...
///         }
///     }
///     fn render_cell(&self, col: Self::Column) -> AnyView {
///         match col {
///             Column::Name => view! { <NameCell ... /> }.into_any(),
///             // ...
///         }
///     }
/// }
/// ```
pub trait DataGridRow: Clone + Send + Sync + 'static {
    /// The ID type for this row (e.g., i32, Uuid).
    type Id: Copy + Eq + std::hash::Hash + Send + Sync + 'static;

    /// The Column enum type for this data grid.
    type Column: DataGridColumn + std::fmt::Display + strum::IntoEnumIterator;

    /// Returns the unique ID for this row.
    fn id(&self) -> Self::Id;

    /// Returns true if this row matches the given filter string.
    fn matches_filter(&self, filter: &str) -> bool;

    /// Returns the string value for a column (used for copy-to-clipboard).
    fn get_value(&self, col: Self::Column) -> String;

    /// Renders the cell content for a column.
    fn render_cell(&self, col: Self::Column) -> AnyView;
}

/// Trait for data grid columns with common utility methods.
/// Extend your Column enum with this trait to get `colindex()`, `is_visible()`, and `is_disabled()`.
///
/// # Example
/// ```ignore
/// impl DataGridColumn for Column {
///     fn colindex(self) -> i32 {
///         self as i32 + 1
///     }
/// }
/// ```
pub trait DataGridColumn: PinnableColumn + AsRef<str> + Send + Sync + 'static {
    /// Returns the 1-based column index for aria-colindex.
    /// Typically implemented as `self as i32 + 1` for enums.
    fn colindex(self) -> i32;

    /// Returns whether this column is disabled (cannot be toggled in the view selector).
    /// Override this method to specify which columns should always be visible.
    fn is_disabled(self) -> bool {
        false
    }

    /// Returns a CSS-safe column name (removes spaces).
    /// Used for CSS variable names like `--col-IsActive-size`.
    fn css_safe_name(self) -> String {
        self.as_ref().chars().filter(|c| *c != ' ').collect()
    }

    /// Returns a signal indicating if column should be shown (not pinned AND visible).
    fn is_visible(
        self,
        pinned_columns_signal: RwSignal<HashSet<Self>>,
        visible_columns_signal: RwSignal<HashSet<String>>,
    ) -> Signal<bool> {
        Signal::derive(move || {
            !pinned_columns_signal.with(|p| p.contains(&self))
                && visible_columns_signal.with(|v| v.contains(self.as_ref()))
        })
    }
}

/// Calculate the left position for a pinned column based on which columns are pinned before it.
/// Starts at 60px to account for the checkbox column.
pub fn get_pinned_left_position<C: PinnableColumn + 'static>(col: C, pinned: &HashSet<C>) -> i32 {
    let mut left = 60; // Start after checkbox (60px)
    for (c, width) in C::pinnable_columns() {
        if *c == col {
            break;
        }
        if pinned.contains(c) {
            left += width;
        }
    }
    left
}

/// Get the width for a pinnable column, or 150 as default if not found.
pub fn get_column_width<C: PinnableColumn + 'static>(col: C) -> i32 {
    C::pinnable_columns().iter().find(|(c, _)| *c == col).map(|(_, w)| *w).unwrap_or(150)
}

/// Generates CSS custom properties for column sizes from pinnable columns.
/// Includes max-height for proper viewport sizing.
/// Use with `LazyLock` to compute once: `static GRID_STYLE: LazyLock<String> = LazyLock::new(generate_grid_style::<Column>);`
pub fn generate_grid_style<C: PinnableColumn + AsRef<str> + 'static>() -> String {
    let mut style = String::from("--header-Select-size: 60; --col-Select-size: 60; ");
    for (col, width) in C::pinnable_columns() {
        // Remove spaces for CSS-safe variable names (e.g., "Is Active" -> "IsActive")
        let name: String = col.as_ref().chars().filter(|c| *c != ' ').collect();
        style.push_str(&format!("--header-{name}-size: {width}; --col-{name}-size: {width}; "));
    }
    style.push_str("max-height: calc(100vh - 16rem);");
    style
}

/// Returns columns that are both pinned AND visible for rendering.
pub fn get_pinned_visible_columns<C>(
    pinned_columns_signal: RwSignal<HashSet<C>>,
    visible_columns_signal: RwSignal<HashSet<String>>,
) -> Vec<(C, i32)>
where
    C: PinnableColumn + AsRef<str> + Copy + Eq + std::hash::Hash + Send + Sync + 'static,
{
    C::pinnable_columns()
        .iter()
        .filter(|(col, _)| {
            pinned_columns_signal.with(|p| p.contains(col)) && visible_columns_signal.with(|v| v.contains(col.as_ref()))
        })
        .copied()
        .collect()
}

/* ========================================================== */
/*                     ✨ CLX COMPONENTS ✨                   */
/* ========================================================== */

mod components {
    use super::*;
    clx! {GridWrapper, div, "flex relative flex-col w-full"}
    clx! {GridCellContent, span, ""}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Grid(
    children: Children,
    #[prop(into)] rowcount: Signal<i32>,
    colcount: i32,
    style: &'static str,
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<html::Div>>,
) -> impl IntoView {
    // NOTE: Avoid `select-none` here to allow text selection via double-click
    let merged_class = tw_merge!("grid overflow-auto relative rounded-md border focus:outline-none", class);

    view! {
        <div
            node_ref=node_ref.unwrap_or_default()
            role="grid"
            data-name="DataGrid"
            class=merged_class
            aria-label="Data grid"
            aria-rowcount=move || rowcount.get().to_string()
            aria-colcount=colcount.to_string()
            tabindex="0"
            style=style
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridBody(
    children: Children,
    #[prop(into)] style: Signal<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!("grid relative", class);

    view! {
        <div
            role="rowgroup"
            data-name="GridBody"
            class=merged_class
            aria-label="Grid Body"
            tabindex="0"
            style=move || style.get()
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ VIRTUALIZED GRID ✨                 */
/* ========================================================== */

/// A Grid wrapper that provides virtual scrolling context.
/// Use with `VirtualizedGridBody` and `VirtualFor` for automatic virtualization.
///
/// # Example
/// ```ignore
/// let total_rows = Signal::derive(move || data.get().len());
///
/// <VirtualizedGrid total_rows rowcount=1000 colcount=10 style=GRID_STYLE.as_str()>
///     <GridHeader ... />
///     <VirtualizedGridBody>
///         <VirtualFor
///             data=Signal::derive(move || data.get())
///             key=|row| row.id
///             children=move |idx, row| view! { <GridRow ... /> }
///         />
///     </VirtualizedGridBody>
/// </VirtualizedGrid>
/// ```
#[component]
pub fn VirtualizedGrid(
    children: Children,
    #[prop(into)] total_rows: Signal<usize>,
    #[prop(into)] rowcount: Signal<i32>,
    colcount: i32,
    style: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let grid_ref = NodeRef::<html::Div>::new();
    let virtual_scroll = use_virtual_scroll(grid_ref, total_rows);

    // Provide context for child components
    provide_context(virtual_scroll);

    view! {
        <Grid rowcount=rowcount colcount=colcount style=style class=class node_ref=grid_ref>
            {children()}
        </Grid>
    }
}

/// Grid body that automatically uses virtual scroll context for height.
/// Must be used within a `VirtualizedGrid`.
#[component]
pub fn VirtualizedGridBody(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let virtual_scroll = expect_context::<VirtualScrollState>();

    let merged_class = tw_merge!("grid relative", class);

    view! {
        <div
            role="rowgroup"
            data-name="GridBody"
            class=merged_class
            aria-label="Grid Body"
            tabindex="0"
            style=move || format!("height: {}px;", virtual_scroll.total_height.get())
        >
            {children()}
        </div>
    }
}

/// A virtualized For loop that only renders visible rows.
/// Must be used within a `VirtualizedGrid`.
///
/// NOTE: The inner `<For>` is keyed by the item's key (not by index) to ensure
/// rows re-render when data changes. This fixes issues where deleting rows
/// would not update the UI because Leptos reuses views with matching keys.
#[component]
pub fn VirtualFor<T, K, KF, CF, CV>(#[prop(into)] data: Signal<Vec<T>>, key: KF, children: CF) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    K: Eq + std::hash::Hash + Clone + Send + 'static,
    KF: Fn(&T) -> K + Clone + Send + Sync + 'static,
    CF: Fn(usize, T) -> CV + Clone + Send + Sync + 'static,
    CV: IntoView + 'static,
{
    let virtual_scroll = expect_context::<VirtualScrollState>();

    let key_fn = StoredValue::new(key);

    view! {
        <For
            each=move || {
                let start = virtual_scroll.start_index.get();
                let end = virtual_scroll.end_index.get();
                data.with(|rows| {
                    (start..end).filter_map(|idx| rows.get(idx).map(|row| (idx, row.clone()))).collect::<Vec<_>>()
                })
            }
            // Key by (index, item_key) so views update when items move position
            key=move |(idx, row): &(usize, T)| {
                let k = key_fn.with_value(|kf| kf(row));
                (*idx, k)
            }
            children=move |(idx, row)| { children.clone()(idx, row).into_any() }
        />
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Grid row with absolute positioning for virtual scrolling.
/// NOTE: `index` must be a Signal to ensure translateY updates when rows are reordered
/// (e.g., during sorting). With keyed `<For>` loops, Leptos reuses DOM elements,
/// so static index values would cause rows to stay in their original visual positions.
#[component]
pub fn GridRow(
    children: Children,
    rowindex: usize,
    #[prop(into)] index: Signal<usize>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!("flex absolute w-full border-b", class);

    view! {
        <div
            role="row"
            data-name="GridRow"
            aria-rowindex=rowindex.to_string()
            aria-selected="false"
            data-index=move || index.get().to_string()
            class=merged_class
            tabindex="-1"
            // Performance: content-visibility:auto skips rendering for off-screen rows
            style=move || {
                let translate_y = index.get() * 36;
                format!(
                    "height: 36px; transform: translateY({translate_y}px); content-visibility: auto; contain-intrinsic-size: auto 36px;",
                )
            }
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

/// Z-index for pinned columns. Must be higher than TableSeparator's z-50.
const PINNED_Z_INDEX: i32 = 51;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridCellWrapper(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    // TODO. slot=checkbox.

    let merged_class = tw_merge!(
        "py-1.5 px-2 text-sm text-left cursor-default outline-none size-full has-data-[slot=checkbox]:pt-2.5 **:data-[name=GridCellContent]:line-clamp-1",
        class
    );

    view! {
        <div data-name="GridCellWrapper" class=merged_class tabindex="-1">
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridHeaderCell(
    children: Children,
    colindex: i32,
    #[prop(into)] column: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] visible: Option<Signal<bool>>,
) -> impl IntoView {
    let merged_class = tw_merge!("relative border-r opacity-100 bg-background data-[visible=false]:hidden", class);

    let formatted_style = format!("width: calc(var(--header-{column}-size) * 1px);");

    // TODO. aria-sort.

    view! {
        <div
            role="columnheader"
            aria-sort="none"
            aria-colindex=colindex.to_string()
            class=merged_class
            data-name="GridHeaderCell"
            data-visible=move || visible.map(|v| v.get()).unwrap_or(true).to_string()
            tabindex="-1"
            style=formatted_style
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridCell(
    children: Children,
    colindex: i32,
    #[prop(into)] column: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] visible: Option<Signal<bool>>,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] current: Signal<bool>,
    #[prop(optional, into)] in_range: Signal<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] on_contextmenu: Option<Callback<()>>,
    #[prop(optional)] on_mousedown: Option<Callback<()>>,
    #[prop(optional)] on_mouseenter: Option<Callback<()>>,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "relative border-r opacity-100 bg-background select-none data-[visible=false]:hidden aria-selected:*:ring-2 aria-selected:*:ring-ring aria-selected:*:ring-inset aria-current:*:bg-neutral-400/20",
        class
    );

    let formatted_style = format!("width: calc(var(--col-{column}-size) * 1px);");

    view! {
        <div
            role="gridcell"
            aria-colindex=colindex.to_string()
            aria-selected=move || active.get().to_string()
            aria-current=move || (current.get() || in_range.get()).to_string()
            class=merged_class
            data-name="GridCell"
            data-visible=move || visible.map(|v| v.get()).unwrap_or(true).to_string()
            tabindex="-1"
            style=formatted_style
            on:click=move |_| {
                if let Some(cb) = on_click {
                    cb.run(());
                }
            }
            on:contextmenu=move |_| {
                if let Some(cb) = on_contextmenu {
                    cb.run(());
                }
            }
            on:mousedown=move |ev| {
                if ev.button() == 0 && let Some(cb) = on_mousedown {
                    cb.run(());
                }
            }
            on:mouseenter=move |_| {
                if let Some(cb) = on_mouseenter {
                    cb.run(());
                }
            }
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky select header cell (checkbox column header).
#[component]
pub fn GridSelectHeaderCell(children: Children) -> impl IntoView {
    view! {
        <div
            role="columnheader"
            aria-colindex="1"
            data-slot="grid-header-cell"
            tabindex="-1"
            class="relative"
            style=format!(
                "left: 0px; position: sticky; background: var(--background); width: calc(var(--header-Select-size) * 1px); z-index: {PINNED_Z_INDEX};",
            )
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky select cell (checkbox column).
#[component]
pub fn GridSelectCell(children: Children) -> impl IntoView {
    view! {
        <div
            role="gridcell"
            aria-colindex="1"
            data-slot="grid-cell"
            tabindex="-1"
            style=format!(
                "left: 0px; position: sticky; background: var(--background); width: calc(var(--col-Select-size) * 1px); z-index: {PINNED_Z_INDEX};",
            )
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky pinned header cell with dynamic left position.
#[component]
pub fn GridPinnedHeaderCell(children: Children, left: i32, width: i32) -> impl IntoView {
    view! {
        <div
            role="columnheader"
            data-slot="grid-header-cell"
            tabindex="-1"
            class="relative"
            style=format!(
                "left: {left}px; position: sticky; background: var(--background); width: {width}px; z-index: {PINNED_Z_INDEX};",
            )
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky pinned cell with dynamic left position.
#[component]
pub fn GridPinnedCell<C: DataGridColumn + 'static>(
    children: Children,
    col: C,
    pinned_columns_signal: RwSignal<HashSet<C>>,
) -> impl IntoView {
    view! {
        <div
            role="gridcell"
            aria-colindex=col.colindex()
            data-name="GridCell"
            tabindex="-1"
            class="relative border-r opacity-100 bg-background"
            style=move || {
                let left = pinned_columns_signal.with(|p| get_pinned_left_position(col, p));
                let width = get_column_width(col);
                format!(
                    "left: {left}px; position: sticky; background: var(--background); width: {width}px; z-index: {PINNED_Z_INDEX};",
                )
            }
        >
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn TableSeparator(
    #[prop(default = 60)] valuemin: i32,
    #[prop(default = 800)] valuemax: i32,
    valuenow: i32,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "absolute top-0 -right-px z-50 w-0.5 h-full opacity-0 transition-opacity select-none hover:opacity-100 focus:outline-none after:-translate-x-1/2 cursor-ew-resize touch-none bg-border after:absolute after:inset-y-0 after:left-1/2 after:h-full after:w-[18px] after:content-[''] hover:bg-primary focus:bg-primary",
        class
    );

    view! {
        <div
            role="separator"
            data-name="TableSeparator"
            class=merged_class
            aria-orientation="vertical"
            aria-label="Resize Name column"
            aria-valuenow=valuenow.to_string()
            aria-valuemin=valuemin.to_string()
            aria-valuemax=valuemax.to_string()
            tabindex="0"
        ></div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// A header cell with sorting and pinning dropdown menu.
/// Works with any column type that implements `PinnableColumn`.
#[component]
pub fn PinnableSortableHeaderCell<C>(
    column: C,
    #[prop(into)] label: String,
    sort_signal: RwSignal<SortDirection>,
    pinned_columns_signal: RwSignal<HashSet<C>>,
    #[prop(optional)] visible_columns_signal: Option<RwSignal<HashSet<String>>>,
    #[prop(default = false)] is_pinned: bool,
) -> impl IntoView
where
    C: PinnableColumn + AsRef<str> + Send + Sync + 'static,
{
    let width = get_column_width(column);
    let column_name = column.as_ref().to_string();

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="flex gap-2 justify-between items-center p-2 w-full h-full text-sm rounded-none border-0 shadow-none data-[state=open]:bg-accent/40 [&_svg]:size-4 hover:bg-accent/40">
                <div class="flex flex-1 gap-1.5 items-center min-w-0">
                    <span class="truncate">{label}</span>
                </div>
                <ChevronDown class="shrink-0 text-muted-foreground" />
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuRadioGroup value=sort_signal>
                    <DropdownMenuRadioItem value=SortDirection::Asc>
                        <ArrowUpNarrowWide class="text-muted-foreground" />
                        <span>"Sort asc"</span>
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value=SortDirection::Desc>
                        <ArrowDownWideNarrow class="text-muted-foreground" />
                        <span>"Sort desc"</span>
                    </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>
                {move || {
                    (sort_signal.get() != SortDirection::None)
                        .then(|| {
                            view! {
                                <DropdownMenuItem on:click=move |_| sort_signal.set(SortDirection::None)>
                                    <DropdownMenuAction>
                                        <CircleX class="text-muted-foreground" />
                                        <span>"Remove sort"</span>
                                    </DropdownMenuAction>
                                </DropdownMenuItem>
                            }
                        })
                }}
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    {if is_pinned {
                        view! {
                            <DropdownMenuItem on:click=move |_| {
                                pinned_columns_signal
                                    .update(|p| {
                                        p.remove(&column);
                                    })
                            }>
                                <DropdownMenuAction>
                                    <PanelLeftClose class="text-muted-foreground" />
                                    <span>"Unpin from left"</span>
                                </DropdownMenuAction>
                            </DropdownMenuItem>
                        }
                            .into_any()
                    } else {
                        view! {
                            <DropdownMenuItem on:click=move |_| {
                                pinned_columns_signal
                                    .update(|p| {
                                        p.insert(column);
                                    })
                            }>
                                <DropdownMenuAction>
                                    <PanelLeft class="text-muted-foreground" />
                                    <span>"Pin to left"</span>
                                </DropdownMenuAction>
                            </DropdownMenuItem>
                        }
                            .into_any()
                    }}
                </DropdownMenuGroup>
                {visible_columns_signal
                    .map(|signal| {
                        let column_name = column_name.clone();
                        view! {
                            <DropdownMenuGroup>
                                <DropdownMenuItem on:click=move |_| {
                                    signal
                                        .update(|v| {
                                            v.remove(&column_name);
                                        })
                                }>
                                    <DropdownMenuAction>
                                        <EyeOff class="text-muted-foreground" />
                                        <span>"Hide Column"</span>
                                    </DropdownMenuAction>
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        }
                    })}
            </DropdownMenuContent>
        </DropdownMenu>
        <TableSeparator valuenow=width />
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Editable cell content component.
/// Double-click to edit, Enter to save, Escape to cancel.
#[component]
pub fn EditableCellContent<C: DataGridColumn + 'static>(
    row_idx: usize,
    col: C,
    #[prop(into)] value: String,
    #[prop(optional)] on_save: Option<Callback<(usize, C, String)>>,
) -> impl IntoView {
    let ctx = expect_context::<CellEditContext<C>>();

    let value = StoredValue::new(value);
    let on_save = StoredValue::new(on_save);

    let is_editing = Signal::derive(move || ctx.is_editing(row_idx, col));

    view! {
        <Show
            when=is_editing
            fallback=move || {
                view! {
                    <GridCellContent
                        class="cursor-text"
                        on:dblclick=move |_| {
                            ctx.start_edit(row_idx, col, value.get_value());
                        }
                    >
                        {value.get_value()}
                    </GridCellContent>
                }
            }
        >
            <input
                type="text"
                class="py-0 px-0 w-full h-full text-sm bg-transparent border-none outline-none focus:ring-0"
                prop:value=move || ctx.edit_value.get()
                on:input=move |ev| ctx.edit_value.set(event_target_value(&ev))
                on:blur=move |_| {
                    if let Some((row, column, new_value)) = ctx.finish_edit() {
                        on_save
                            .with_value(|cb| {
                                if let Some(cb) = cb {
                                    cb.run((row, column, new_value));
                                }
                            });
                    }
                }
                on:keydown=move |ev: KeyboardEvent| {
                    match ev.key().as_str() {
                        "Enter" => {
                            ev.prevent_default();
                            if let Some((row, column, new_value)) = ctx.finish_edit() {
                                on_save
                                    .with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run((row, column, new_value));
                                        }
                                    });
                            }
                        }
                        "Escape" => {
                            ev.prevent_default();
                            ctx.cancel_edit();
                        }
                        _ => {}
                    }
                }
                autofocus=true
            />
        </Show>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DataGridToolbar(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let merged_class = tw_merge!("flex gap-4 justify-between items-center mb-4", class);

    view! {
        <div data-name="DataGridToolbar" role="toolbar" aria-orientation="horizontal" class=merged_class>
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Generic grid header component that works with any Column type.
/// Handles the select-all checkbox, pinned headers, and non-pinned headers.
#[component]
pub fn GenericGridHeader<C>(
    row_count_signal: Signal<usize>,
    selected_count_signal: Signal<usize>,
    handle_select_all: Callback<bool>,
    sort_signals: StoredValue<HashMap<C, RwSignal<SortDirection>>>,
    pinned_columns_signal: RwSignal<HashSet<C>>,
    visible_columns_signal: RwSignal<HashSet<String>>,
) -> impl IntoView
where
    C: DataGridColumn + std::fmt::Display + 'static,
{
    view! {
        <div role="rowgroup" data-slot="grid-header" class="grid sticky top-0 z-10 border-b bg-background">
            <div role="row" aria-rowindex="1" data-slot="grid-header-row" tabindex="-1" class="flex w-full">
                // Select header (always sticky)
                <GridSelectHeaderCell>
                    <div class="py-1.5 px-3 size-full">
                        <Checkbox
                            checked=Signal::derive(move || {
                                let row_count = row_count_signal.get();
                                row_count > 0 && selected_count_signal.get() == row_count
                            })
                            on_checked_change=handle_select_all
                            aria_label="Select all"
                        />
                    </div>
                </GridSelectHeaderCell>

                // Pinned headers (dynamic loop)
                <For
                    each=move || get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal)
                    key=|(col, _)| *col
                    children=move |(col, _width)| {
                        let sort_signal = sort_signals.with_value(|s| s.get(&col).copied());
                        let Some(sort_signal) = sort_signal else {
                            return view! { <div></div> }.into_any();
                        };
                        let left = pinned_columns_signal.with(|p| get_pinned_left_position(col, p));
                        let width = get_column_width(col);

                        view! {
                            <GridPinnedHeaderCell left=left width=width>
                                <PinnableSortableHeaderCell
                                    column=col
                                    label=col.to_string()
                                    sort_signal=sort_signal
                                    pinned_columns_signal=pinned_columns_signal
                                    visible_columns_signal=visible_columns_signal
                                    is_pinned=true
                                />
                            </GridPinnedHeaderCell>
                        }
                            .into_any()
                    }
                />

                // Non-pinned headers (dynamic loop)
                <For
                    each=move || C::pinnable_columns().to_vec()
                    key=|(col, _)| *col
                    children=move |(col, _width)| {
                        let sort_signal = sort_signals.with_value(|s| s.get(&col).copied());
                        let Some(sort_signal) = sort_signal else {
                            return view! { <div></div> }.into_any();
                        };
                        view! {
                            <GridHeaderCell
                                colindex=col.colindex()
                                column=col.css_safe_name()
                                visible=col.is_visible(pinned_columns_signal, visible_columns_signal)
                            >
                                <PinnableSortableHeaderCell
                                    column=col
                                    label=col.to_string()
                                    sort_signal=sort_signal
                                    pinned_columns_signal=pinned_columns_signal
                                    visible_columns_signal=visible_columns_signal
                                />
                            </GridHeaderCell>
                        }
                            .into_any()
                    }
                />
            </div>
        </div>
    }
}
```
