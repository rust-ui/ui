+++
title = "Data Table"
description = "Powerful data table with advanced features including filtering, sorting, column visibility, and row selection."
tags = ["table"]
is_new = false
image = "/images/thumbnails/table.webp"
image_dark = "/images/thumbnails/table-dark.webp"
+++

<StaticDataTable />

## Installation

<StaticInstallDataTable />

## Usage

```rust
use crate::ui::table::Table;
```

```rust
rsx! {
    DemoDataTable {}
}
```

## Examples

### Basic Data Table

Simple structured data table for displaying rows and columns with consistent styling.

<StaticDataTable />

### Filtered Table

Data table with interactive filtering controls for narrowing large datasets.

<StaticDataTableFilters />

## See Also

- [Table](/components/table)
- [Pagination](/components/pagination)
