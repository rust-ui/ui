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

## Usage

```rust
use crate::ui::table::{
    Table, TableHeader, TableBody, TableRow,
    TableHead, TableCell, TableCaption,
};
```

```rust
rsx! {
    Table {
        TableHeader {
            TableRow {
                TableHead { "Name" }
                TableHead { "Email" }
            }
        }
        TableBody {
            TableRow {
                TableCell { "Alice" }
                TableCell { "alice@example.com" }
            }
        }
    }
}
```

## See Also

- [Card](/components/card)
- [Badge](/components/badge)
