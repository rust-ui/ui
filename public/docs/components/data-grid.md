+++
title = "Data Grid"
description = "High-performance data grid with virtual scrolling, column pinning, cell selection, inline editing, and drag selection."
tags = ["table"]
is_new = false
image = "/images/thumbnails/table.webp"
image_dark = "/images/thumbnails/table-dark.webp"
+++

<StaticDataGrid />



## Installation

<StaticInstallDataGrid />



## Components

**Layout**
- **GridWrapper**: Outer flex container with relative positioning
- **Grid**: Main grid container — sets `role="grid"`, CSS column sizes, and virtual scroll bounds
- **GridBody**: Absolute-positioned row container for virtual scrolling
- **GridRow**: Absolutely-positioned row with `translateY` for virtual placement

**Cells**
- **GridHeaderCell**: Standard header cell — reads `--header-{Column}-size` CSS variable
- **GridPinnedHeaderCell**: Sticky header cell for pinned columns
- **GridSelectHeaderCell**: Header cell with select-all checkbox
- **PinnableSortableHeaderCell**: Header cell with sort toggle and pin/unpin dropdown
- **GridCell**: Standard data cell with selection and drag-selection support
- **GridPinnedCell**: Sticky data cell for pinned columns
- **GridSelectCell**: Data cell with row-selection checkbox
- **GridCellWrapper**: Inner cell wrapper — handles padding, cursor, and line-clamp
- **GridCellContent**: `span` wrapper for cell text content

**Toolbar**
- **DataGridToolbar**: Toolbar with column visibility toggle



## Hooks

- **`use_data_grid_state::<Column>()`** — Sets up cell selection, drag selection, copy signal, and grid wrapper element with click-outside handling. Returns `DataGridState`.
- **`use_drag_selection::<Column>()`** — Range selection via click-drag. Returns `UseDragSelection`.
- **`use_copy_clipboard()`** — Copy-to-clipboard with toast feedback.
- **`use_press_hold(...)`** — Press-and-hold for long-press actions.

> **Note:** Unlike the Leptos version, `grid_wrapper_element` is a `Signal<Option<web_sys::Element>>` set via `onmounted` — there is no `NodeRef`.



## Traits

Implement on your `Column` enum:

- **`PinnableColumn`** — Declares pinnable columns and pixel widths via `pinnable_columns()`
- **`DataGridColumn`** — Provides `colindex()` (1-based), `is_disabled()`, `css_safe_name()`, `is_visible()`
- **`SortableColumn<RowData>`** — Implements `compare()` per column for sort support

Optionally on your row struct:

- **`DataGridRow`** — For use with `GenericDataGrid`: implement `id()`, `matches_filter()`, `get_value()`, `render_cell()`



## Utilities

```rust
use crate::ui::data_grid::{
    generate_grid_style,       // Builds CSS custom properties string from PINNABLE_COLUMNS
    get_column_width,          // Returns pixel width for a column
    get_pinned_left_position,  // Computes sticky left offset for a pinned column
    get_pinned_visible_columns // Returns currently pinned + visible columns in order
};
```

Use `LazyLock` to compute the grid style once at startup:

```rust
static GRID_STYLE: LazyLock<String> = LazyLock::new(generate_grid_style::<Column>);
```



## Usage

```rust
use crate::ui::data_grid::{
    DataGridColumn, DataGridToolbar, Grid, GridBody, GridCell, GridCellContent,
    GridCellWrapper, GridHeaderCell, GridPinnedCell, GridPinnedHeaderCell, GridRow,
    GridSelectCell, GridSelectHeaderCell, GridWrapper, PinnableColumn,
    PinnableSortableHeaderCell, SortDirection, SortableColumn,
    generate_grid_style, get_column_width, get_pinned_left_position,
    get_pinned_visible_columns,
};
use crate::hooks::use_data_grid_state::{DataGridState, use_data_grid_state};
use crate::hooks::use_drag_selection::{UseDragSelection, use_drag_selection};
```

**1. Define your column enum:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, AsRefStr, EnumIter)]
#[strum(serialize_all = "title_case")]
pub enum Column { Select, Name, Email, Status }

impl DataGridColumn for Column {
    fn colindex(self) -> i32 { self as i32 + 1 }
    fn is_disabled(self) -> bool {
        matches!(self, Self::Select | Self::Name)
    }
}

impl PinnableColumn for Column {
    fn pinnable_columns() -> &'static [(Self, i32)] { PINNABLE_COLUMNS }
}

impl SortableColumn<RowData> for Column {
    fn compare(self, a: &RowData, b: &RowData) -> Option<std::cmp::Ordering> {
        match self {
            Self::Name  => Some(a.name.cmp(&b.name)),
            Self::Email => Some(a.email.cmp(&b.email)),
            _           => None,
        }
    }
}

const PINNABLE_COLUMNS: &[(Column, i32)] = &[
    (Column::Name, 180),
    (Column::Email, 240),
    (Column::Status, 160),
];

static GRID_STYLE: LazyLock<String> = LazyLock::new(generate_grid_style::<Column>);
```

**2. Set up state and render:**

```rust
#[component]
fn MyDataGrid() -> Element {
    let rows = use_signal(Vec::<RowData>::new);

    let DataGridState { cell_selection, drag_selection, copy_value_signal, grid_wrapper_element } =
        use_data_grid_state::<Column>();

    let row_count = use_memo(move || rows.read().len());
    let grid_body_height = use_memo(move || format!("height: {}px", row_count() * 36));

    rsx! {
        GridWrapper {
            Grid {
                rowcount: row_count() as i32,
                colcount: Column::iter().count() as i32,
                style: GRID_STYLE.as_str(),
                onmounted: move |e| grid_wrapper_element.set(Some(e.data.element())),
                // header row...
                GridBody {
                    style: grid_body_height(),
                    // For over rows...
                }
            }
        }
    }
}
```



## Features

- **Virtual scrolling**: Absolute-positioned rows with `translateY` — renders only visible rows
- **Column pinning**: Sticky columns with computed `left` offsets and elevated z-index
- **Sorting**: Per-column `SortDirection` signals; `SortableColumn` trait drives comparison
- **Cell selection**: Click to select active cell; right-click for context menu cell
- **Drag selection**: Click-drag to select a range of cells
- **Copy to clipboard**: Right-click context menu or keyboard shortcut via `use_copy_clipboard`
- **Row selection**: Checkbox column with select-all support
- **Column visibility**: Toggle visible columns via `DataGridToolbar`



## See Also

- [Data Table](/docs/data_table)
- [Table](/docs/table)
- [Checkbox](/docs/checkbox)
- [Context Menu](/docs/context_menu)
- [Multi Select](/docs/multi_select)
