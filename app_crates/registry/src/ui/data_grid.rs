use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use dioxus::prelude::*;
use icons::{ArrowDownWideNarrow, ArrowUpNarrowWide, ChevronDown, CircleX, EyeOff, PanelLeft, PanelLeftClose};
use tw_merge::tw_merge;

use crate::hooks::use_cell_edit::CellEditContext;
use crate::hooks::use_virtual_scroll::{VirtualScrollState, use_virtual_scroll};
use crate::ui::checkbox::Checkbox;
use crate::ui::dropdown_menu::{
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
    fn render_cell(&self, col: Self::Column) -> Element;
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

    /// Returns whether column should be shown (not pinned AND visible).
    fn is_visible(
        self,
        pinned_columns_signal: Signal<HashSet<Self>>,
        visible_columns_signal: Signal<HashSet<String>>,
    ) -> bool {
        !pinned_columns_signal.read().contains(&self) && visible_columns_signal.read().contains(self.as_ref())
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
    pinned_columns_signal: Signal<HashSet<C>>,
    visible_columns_signal: Signal<HashSet<String>>,
) -> Vec<(C, i32)>
where
    C: PinnableColumn + AsRef<str> + Copy + Eq + std::hash::Hash + Send + Sync + 'static,
{
    C::pinnable_columns()
        .iter()
        .filter(|(col, _)| {
            pinned_columns_signal.read().contains(col) && visible_columns_signal.read().contains(col.as_ref())
        })
        .copied()
        .collect()
}

/* ========================================================== */
/*                     ✨ COMPONENTS ✨                       */
/* ========================================================== */

#[component]
pub fn GridWrapper(children: Element) -> Element {
    rsx! { div { "data-name": "GridWrapper", class: "flex relative flex-col w-full", {children} } }
}

#[component]
pub fn GridCellContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "GridCellContent", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Grid(
    children: Element,
    rowcount: Signal<i32>,
    colcount: i32,
    style: &'static str,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    // NOTE: Avoid `select-none` here to allow text selection via double-click
    let merged_class =
        tw_merge!("grid overflow-auto relative rounded-md border focus:outline-none", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            role: "grid",
            "data-name": "DataGrid",
            class: "{merged_class}",
            "aria-label": "Data grid",
            "aria-rowcount": "{rowcount()}",
            "aria-colcount": "{colcount}",
            tabindex: "0",
            style: "{style}",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridBody(children: Element, style: Signal<String>, #[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("grid relative", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            role: "rowgroup",
            "data-name": "GridBody",
            class: "{merged_class}",
            "aria-label": "Grid Body",
            tabindex: "0",
            style: "{style()}",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ VIRTUALIZED GRID ✨                 */
/* ========================================================== */

#[component]
pub fn VirtualizedGrid(
    children: Element,
    total_rows: ReadSignal<usize>,
    rowcount: Signal<i32>,
    colcount: i32,
    style: &'static str,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let mut grid_element: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let virtual_scroll = use_virtual_scroll(grid_element.into(), total_rows);

    provide_context(virtual_scroll);

    rsx! {
        div {
            onmounted: move |event| {
                if let Some(element) = event.data().downcast::<web_sys::Element>().cloned() {
                    grid_element.set(Some(element));
                }
            },
            Grid {
                rowcount,
                colcount,
                style,
                class,
                {children}
            }
        }
    }
}

#[component]
pub fn VirtualizedGridBody(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let virtual_scroll = consume_context::<VirtualScrollState>();
    let merged_class = tw_merge!("grid relative", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            role: "rowgroup",
            "data-name": "GridBody",
            class: "{merged_class}",
            "aria-label": "Grid Body",
            tabindex: "0",
            style: format!("height: {}px;", (virtual_scroll.total_height)()),
            {children}
        }
    }
}

/// A virtualized For loop that only renders visible rows.
/// Must be used within a component that provides `VirtualScrollState` context.
///
/// # Usage
/// ```rust
/// {VirtualFor(data_signal, move |idx, row| rsx! { GridRow { ... } })}
/// ```
#[allow(non_snake_case)]
pub fn VirtualFor<T: Clone + 'static>(data: Signal<Vec<T>>, render: impl Fn(usize, T) -> Element) -> Element {
    let virtual_scroll = consume_context::<VirtualScrollState>();
    let start = (virtual_scroll.start_index)();
    let end = (virtual_scroll.end_index)();
    let visible: Vec<(usize, T)> = {
        let rows = data.read();
        (start..end).filter_map(|idx| rows.get(idx).map(|row| (idx, row.clone()))).collect()
    };
    rsx! {
        for (idx, row) in visible {
            {render(idx, row)}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Grid row with absolute positioning for virtual scrolling.
/// NOTE: `index` must be a Signal to ensure translateY updates when rows are reordered
/// (e.g., during sorting). With keyed loops, Dioxus reuses DOM elements,
/// so static index values would cause rows to stay in their original visual positions.
#[component]
pub fn GridRow(
    children: Element,
    rowindex: usize,
    index: Signal<usize>,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let merged_class = tw_merge!("flex absolute w-full border-b", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            role: "row",
            "data-name": "GridRow",
            "aria-rowindex": "{rowindex}",
            "aria-selected": "false",
            "data-index": "{index()}",
            class: "{merged_class}",
            tabindex: "-1",
            // Performance: content-visibility:auto skips rendering for off-screen rows
            style: {
                let translate_y = index() * 36;
                format!(
                    "height: 36px; transform: translateY({translate_y}px); content-visibility: auto; contain-intrinsic-size: auto 36px;",
                )
            },
            {children}
        }
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
pub fn GridCellWrapper(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!(
        "py-1.5 px-2 text-sm text-left cursor-default outline-none size-full has-data-[slot=checkbox]:pt-2.5 **:data-[name=GridCellContent]:line-clamp-1",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { "data-name": "GridCellWrapper", class: "{merged_class}", tabindex: "-1", {children} }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridHeaderCell(
    children: Element,
    colindex: i32,
    #[props(into)] column: String,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] visible: Option<bool>,
) -> Element {
    let merged_class = tw_merge!(
        "relative border-r opacity-100 bg-background data-[visible=false]:hidden",
        class.as_deref().unwrap_or("")
    );

    let formatted_style = format!("width: calc(var(--header-{column}-size) * 1px);");
    let visible_str = visible.unwrap_or(true).to_string();

    rsx! {
        div {
            role: "columnheader",
            "aria-sort": "none",
            "aria-colindex": "{colindex}",
            class: "{merged_class}",
            "data-name": "GridHeaderCell",
            "data-visible": "{visible_str}",
            tabindex: "-1",
            style: "{formatted_style}",
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn GridCell(
    children: Element,
    colindex: i32,
    #[props(into)] column: String,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] visible: Option<bool>,
    #[props(default = false)] active: bool,
    #[props(default = false)] current: bool,
    #[props(default = false)] in_range: bool,
    #[props(optional)] on_click: Option<EventHandler<()>>,
    #[props(optional)] on_contextmenu: Option<EventHandler<()>>,
    #[props(optional)] on_mousedown: Option<EventHandler<()>>,
    #[props(optional)] on_mouseenter: Option<EventHandler<()>>,
) -> Element {
    let merged_class = tw_merge!(
        "relative border-r opacity-100 bg-background select-none data-[visible=false]:hidden aria-selected:*:ring-2 aria-selected:*:ring-ring aria-selected:*:ring-inset aria-current:*:bg-neutral-400/20",
        class.as_deref().unwrap_or("")
    );

    let formatted_style = format!("width: calc(var(--col-{column}-size) * 1px);");
    let visible_str = visible.unwrap_or(true).to_string();
    let active_str = active.to_string();
    let current_str = (current || in_range).to_string();

    rsx! {
        div {
            role: "gridcell",
            "aria-colindex": "{colindex}",
            "aria-selected": "{active_str}",
            "aria-current": "{current_str}",
            class: "{merged_class}",
            "data-name": "GridCell",
            "data-visible": "{visible_str}",
            tabindex: "-1",
            style: "{formatted_style}",
            onclick: move |_| {
                if let Some(cb) = &on_click {
                    cb.call(());
                }
            },
            oncontextmenu: move |_| {
                if let Some(cb) = &on_contextmenu {
                    cb.call(());
                }
            },
            onmousedown: move |ev| {
                if ev.data().trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary)
                    && let Some(cb) = &on_mousedown {
                        cb.call(());
                    }
            },
            onmouseenter: move |_| {
                if let Some(cb) = &on_mouseenter {
                    cb.call(());
                }
            },
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky select header cell (checkbox column header).
#[component]
pub fn GridSelectHeaderCell(children: Element) -> Element {
    rsx! {
        div {
            role: "columnheader",
            "aria-colindex": "1",
            "data-slot": "grid-header-cell",
            tabindex: "-1",
            class: "relative",
            style: format!(
                "left: 0px; position: sticky; background: var(--background); width: calc(var(--header-Select-size) * 1px); z-index: {PINNED_Z_INDEX};",
            ),
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky select cell (checkbox column).
#[component]
pub fn GridSelectCell(children: Element) -> Element {
    rsx! {
        div {
            role: "gridcell",
            "aria-colindex": "1",
            "data-slot": "grid-cell",
            tabindex: "-1",
            style: format!(
                "left: 0px; position: sticky; background: var(--background); width: calc(var(--col-Select-size) * 1px); z-index: {PINNED_Z_INDEX};",
            ),
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky pinned header cell with dynamic left position.
#[component]
pub fn GridPinnedHeaderCell(children: Element, left: i32, width: i32) -> Element {
    rsx! {
        div {
            role: "columnheader",
            "data-slot": "grid-header-cell",
            tabindex: "-1",
            class: "relative",
            style: format!(
                "left: {left}px; position: sticky; background: var(--background); width: {width}px; z-index: {PINNED_Z_INDEX};",
            ),
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Sticky pinned cell with dynamic left position.
#[component]
pub fn GridPinnedCell<C: DataGridColumn + 'static>(
    children: Element,
    col: C,
    pinned_columns_signal: Signal<HashSet<C>>,
) -> Element {
    let width = get_column_width(col);

    rsx! {
        div {
            role: "gridcell",
            "aria-colindex": "{col.colindex()}",
            "data-name": "GridCell",
            tabindex: "-1",
            class: "relative border-r opacity-100 bg-background",
            style: {
                let left = get_pinned_left_position(col, &pinned_columns_signal.read());
                format!(
                    "left: {left}px; position: sticky; background: var(--background); width: {width}px; z-index: {PINNED_Z_INDEX};",
                )
            },
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn TableSeparator(
    #[props(default = 60)] valuemin: i32,
    #[props(default = 800)] valuemax: i32,
    valuenow: i32,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let merged_class = tw_merge!(
        "absolute top-0 -right-px z-50 w-0.5 h-full opacity-0 transition-opacity select-none hover:opacity-100 focus:outline-none after:-translate-x-1/2 cursor-ew-resize touch-none bg-border after:absolute after:inset-y-0 after:left-1/2 after:h-full after:w-[18px] after:content-[''] hover:bg-primary focus:bg-primary",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            role: "separator",
            "data-name": "TableSeparator",
            class: "{merged_class}",
            "aria-orientation": "vertical",
            "aria-label": "Resize Name column",
            "aria-valuenow": "{valuenow}",
            "aria-valuemin": "{valuemin}",
            "aria-valuemax": "{valuemax}",
            tabindex: "0",
        }
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
    #[props(into)] label: String,
    sort_signal: Signal<SortDirection>,
    pinned_columns_signal: Signal<HashSet<C>>,
    #[props(optional)] visible_columns_signal: Option<Signal<HashSet<String>>>,
    #[props(default = false)] is_pinned: bool,
) -> Element
where
    C: PinnableColumn + AsRef<str> + Send + Sync + 'static,
{
    let width = get_column_width(column);
    let column_name = column.as_ref().to_string();
    let mut sort_signal = sort_signal;
    let mut pinned_columns_signal = pinned_columns_signal;

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { class: "flex gap-2 justify-between items-center p-2 w-full h-full text-sm rounded-none border-0 shadow-none data-[state=open]:bg-accent/40 [&_svg]:size-4 hover:bg-accent/40",
                div { class: "flex flex-1 gap-1.5 items-center min-w-0",
                    span { class: "truncate", "{label}" }
                }
                ChevronDown { class: "shrink-0 text-muted-foreground" }
            }
            DropdownMenuContent {
                DropdownMenuRadioGroup { value: sort_signal,
                    DropdownMenuRadioItem { value: SortDirection::Asc,
                        ArrowUpNarrowWide { class: "text-muted-foreground" }
                        span { "Sort asc" }
                    }
                    DropdownMenuRadioItem { value: SortDirection::Desc,
                        ArrowDownWideNarrow { class: "text-muted-foreground" }
                        span { "Sort desc" }
                    }
                }
                if sort_signal() != SortDirection::None {
                    DropdownMenuItem {
                        onclick: move |_| sort_signal.set(SortDirection::None),
                        DropdownMenuAction {
                            CircleX { class: "text-muted-foreground" }
                            span { "Remove sort" }
                        }
                    }
                }
                DropdownMenuSeparator {}
                DropdownMenuGroup {
                    if is_pinned {
                        DropdownMenuItem {
                            onclick: move |_| {
                                pinned_columns_signal.with_mut(|p| { p.remove(&column); })
                            },
                            DropdownMenuAction {
                                PanelLeftClose { class: "text-muted-foreground" }
                                span { "Unpin from left" }
                            }
                        }
                    } else {
                        DropdownMenuItem {
                            onclick: move |_| {
                                pinned_columns_signal.with_mut(|p| { p.insert(column); })
                            },
                            DropdownMenuAction {
                                PanelLeft { class: "text-muted-foreground" }
                                span { "Pin to left" }
                            }
                        }
                    }
                }
                if let Some(mut vis_signal) = visible_columns_signal {
                    DropdownMenuGroup {
                        DropdownMenuItem {
                            onclick: move |_| {
                                let col_name = column_name.clone();
                                vis_signal.with_mut(|v| { v.remove(&col_name); })
                            },
                            DropdownMenuAction {
                                EyeOff { class: "text-muted-foreground" }
                                span { "Hide Column" }
                            }
                        }
                    }
                }
            }
        }
        TableSeparator { valuenow: width }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn EditableCellContent<C: DataGridColumn + 'static>(
    row_idx: usize,
    col: C,
    value: String,
    #[props(optional)] on_save: Option<EventHandler<(usize, C, String)>>,
) -> Element {
    let mut ctx = consume_context::<CellEditContext<C>>();
    let is_editing = ctx.is_editing(row_idx, col);
    let initial_value = value.clone();

    if is_editing {
        rsx! {
            input {
                r#type: "text",
                class: "py-0 px-0 w-full h-full text-sm bg-transparent border-none outline-none focus:ring-0",
                value: "{(ctx.edit_value)()}",
                onmounted: move |ev| {
                    spawn(async move {
                        let _ = ev.set_focus(true).await;
                    });
                },
                oninput: move |ev| ctx.edit_value.set(ev.value()),
                onblur: move |_| {
                    if let Some((row, column, new_value)) = ctx.finish_edit()
                        && let Some(cb) = &on_save {
                            cb.call((row, column, new_value));
                        }
                },
                onkeydown: move |ev| {
                    match ev.data().key() {
                        Key::Enter => {
                            ev.prevent_default();
                            if let Some((row, column, new_value)) = ctx.finish_edit()
                                && let Some(cb) = &on_save {
                                    cb.call((row, column, new_value));
                                }
                        }
                        Key::Escape => {
                            ev.prevent_default();
                            ctx.cancel_edit();
                        }
                        _ => {}
                    }
                }
            }
        }
    } else {
        rsx! {
            span {
                "data-name": "GridCellContent",
                class: "cursor-text",
                ondoubleclick: move |_| {
                    ctx.start_edit(row_idx, col, initial_value.clone());
                },
                "{value}"
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DataGridToolbar(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("flex gap-4 justify-between items-center mb-4", class.as_deref().unwrap_or(""));

    rsx! {
        div { "data-name": "DataGridToolbar", role: "toolbar", "aria-orientation": "horizontal", class: "{merged_class}",
            {children}
        }
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
    handle_select_all: EventHandler<bool>,
    sort_signals: Signal<HashMap<C, Signal<SortDirection>>>,
    pinned_columns_signal: Signal<HashSet<C>>,
    visible_columns_signal: Signal<HashSet<String>>,
) -> Element
where
    C: DataGridColumn + std::fmt::Display + 'static,
{
    let all_checked = {
        let row_count = row_count_signal();
        row_count > 0 && selected_count_signal() == row_count
    };

    rsx! {
        div { role: "rowgroup", "data-slot": "grid-header", class: "grid sticky top-0 z-10 border-b bg-background",
            div { role: "row", "aria-rowindex": "1", "data-slot": "grid-header-row", tabindex: "-1", class: "flex w-full",
                // Select header (always sticky)
                GridSelectHeaderCell {
                    div { class: "py-1.5 px-3 size-full",
                        Checkbox {
                            checked: all_checked,
                            on_change: move |checked| handle_select_all.call(checked),
                        }
                    }
                }

                // Pinned headers (dynamic loop)
                for (col, _width) in get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal) {
                    {
                        let sort_signal = sort_signals.read().get(&col).copied();
                        let left = get_pinned_left_position(col, &pinned_columns_signal.read());
                        let width = get_column_width(col);
                        if let Some(sort_signal) = sort_signal {
                            rsx! {
                                GridPinnedHeaderCell { left, width,
                                    PinnableSortableHeaderCell {
                                        column: col,
                                        label: col.to_string(),
                                        sort_signal,
                                        pinned_columns_signal,
                                        visible_columns_signal,
                                        is_pinned: true,
                                    }
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }

                // Non-pinned headers (dynamic loop)
                for (col, _width) in C::pinnable_columns().iter().copied() {
                    {
                        let sort_signal = sort_signals.read().get(&col).copied();
                        let is_visible = col.is_visible(pinned_columns_signal, visible_columns_signal);
                        if let Some(sort_signal) = sort_signal {
                            rsx! {
                                GridHeaderCell {
                                    colindex: col.colindex(),
                                    column: col.css_safe_name(),
                                    visible: is_visible,
                                    PinnableSortableHeaderCell {
                                        column: col,
                                        label: col.to_string(),
                                        sort_signal,
                                        pinned_columns_signal,
                                        visible_columns_signal,
                                    }
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }
            }
        }
    }
}
