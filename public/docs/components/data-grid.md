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

The Data Grid is composed of layout components, header/cell components, and supporting hooks:

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
- **EditableCellContent**: Double-click to edit cell content

**Toolbar**
- **DataGridToolbar**: Toolbar with column visibility toggle



## Hooks

- **`use_data_grid_state::<Column>()`** — Sets up cell selection, drag selection, copy signal, and grid wrapper ref with click-outside handling. Returns `DataGridState`.
- **`use_column_state(PINNABLE_COLUMNS)`** — Manages sort signals, pinned columns, and visible columns. Returns `ColumnState`.
- **`use_cell_edit::<Column>()`** — Provides double-click inline editing via `CellEditContext`.
- **`use_copy_clipboard(None)`** — Copy-to-clipboard with toast feedback.
- **`use_press_hold(...)`** — Press-and-hold for long-press actions (e.g. mobile delete).



## Traits

You must implement these traits on your `Column` enum:

- **`PinnableColumn`** — Declares pinnable columns and their pixel widths via `pinnable_columns()`
- **`DataGridColumn`** — Provides `colindex()` (1-based), `is_disabled()`, `css_safe_name()`, `is_visible()`
- **`SortableColumn<RowData>`** — Implements `compare()` per column for sort support

And optionally on your row struct:

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
    DataGridColumn, DataGridToolbar, EditableCellContent, Grid, GridBody, GridCell,
    GridCellWrapper, GridHeaderCell, GridPinnedCell, GridPinnedHeaderCell, GridRow,
    GridSelectCell, GridSelectHeaderCell, GridWrapper, PinnableColumn,
    PinnableSortableHeaderCell, SortDirection, SortableColumn,
    generate_grid_style, get_column_width, get_pinned_left_position,
    get_pinned_visible_columns,
};
use crate::hooks::use_data_grid_state::{DataGridState, use_data_grid_state};
use crate::hooks::use_column_state::{ColumnState, use_column_state};
use crate::hooks::use_cell_edit::use_cell_edit;
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
pub fn MyDataGrid() -> impl IntoView {
    let rows = RwSignal::new(Vec::<RowData>::new());

    let ColumnState { sort_signals, pinned_columns_signal, visible_columns_signal } =
        use_column_state(PINNABLE_COLUMNS);

    let DataGridState { cell_selection, drag_selection, copy_value_signal, grid_wrapper_ref } =
        use_data_grid_state::<Column>();

    let _cell_edit = use_cell_edit::<Column>();

    let row_count = Signal::derive(move || rows.with(|r| r.len()));
    let grid_body_height = Signal::derive(move || {
        format!("height: {}px", row_count.get() * 36)
    });

    view! {
        <GridWrapper>
            <Grid
                rowcount=Signal::derive(move || row_count.get() as i32)
                colcount=Column::iter().count() as i32
                style=GRID_STYLE.as_str()
            >
                // header row...
                <GridBody style=grid_body_height>
                    // <For> over rows...
                </GridBody>
            </Grid>
        </GridWrapper>
    }
}
```



## Features

- **Virtual scrolling**: Absolute-positioned rows with `translateY` — renders only visible rows
- **Column pinning**: Sticky columns with computed `left` offsets and elevated z-index
- **Sorting**: Per-column `SortDirection` signals; `SortableColumn` trait drives comparison
- **Cell selection**: Click to select active cell; right-click for context menu cell
- **Drag selection**: Click-drag to select a range of cells
- **Inline editing**: Double-click cell to edit via `use_cell_edit` / `EditableCellContent`
- **Copy to clipboard**: Right-click context menu or keyboard shortcut via `use_copy_clipboard`
- **Row selection**: Checkbox column with select-all support
- **Column visibility**: Toggle visible columns via `DataGridToolbar` + `MultiSelect`
- **Keyboard navigation**: Arrow keys move active cell; `Tab` advances focus



## See Also

- [Data Table](/docs/components/data-table)
- [Table](/docs/components/table)
- [Checkbox](/docs/components/checkbox)
- [Context Menu](/docs/components/context-menu)
- [Multi Select](/docs/components/multi-select)
