+++
title = "Table"
description = "Rust/UI component that displays a table with header, body and footer."
tags = ["table"]
is_new = false
image = "/images/thumbnails/table.webp"
image_dark = "/images/thumbnails/table-dark.webp"
+++


<StaticTable />





## Installation

<StaticInstallTable />



## Components

The Table component is composed of several subcomponents:

- **Table**: Main table wrapper component
- **TableHeader**: Header section containing column headings
- **TableBody**: Body section containing data rows
- **TableFooter**: Footer section for summary or totals
- **TableRow**: Individual table row container
- **TableHead**: Header cell for column titles
- **TableCell**: Data cell for row content
- **TableCaption**: Accessible caption describing the table



## Usage

```rust
use crate::components::ui::table::{
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableFooter,
    TableHead,
    TableHeader,
    TableRow,
};
```

```rust
<Table>
    <TableCaption>"Table Caption"</TableCaption>
    <TableHeader>
        <TableRow>
            <TableHead>"Header 1"</TableHead>
            <TableHead>"Header 2"</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>"Cell 1"</TableCell>
            <TableCell>"Cell 2"</TableCell>
        </TableRow>
    </TableBody>
</Table>
```


### RTL

Data table with Arabic column headers and cell content. Text alignment and logical padding adapt to right-to-left reading direction.

<StaticTableRtl />

## See Also

- [Data Table](/docs/components/data-table)
- [Pagination](/docs/components/pagination)
- [Scroll Area](/docs/components/scroll-area)
