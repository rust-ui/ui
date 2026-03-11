---
title: "Table"
name: "table"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/table.rs"
description: "Rust/UI component that displays a table with header, body and footer."
tags: ["table"]
---

# Table

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add table
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {TableWrapper, div, "overflow-hidden rounded-md border"}
    clx! {Table, table, "w-full max-w-7xl text-sm caption-bottom"}
    clx! {TableCaption, caption, "mt-4 text-sm text-muted-foreground"}
    clx! {TableHeader, thead, "[&_tr]:border-b"}
    clx! {TableRow, tr, "border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50"}
    clx! {TableHead, th, "h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]"}
    clx! {TableBody, tbody, "[&_tr:last-child]:border-0"}
    clx! {TableCell, td, "p-4 align-middle [&:has([role=checkbox])]:pr-0  &:has([role=checkbox])]:pl-3"}
    clx! {TableFooter, tfoot, "font-medium border border-t bg-muted/50 [&>tr]:last:border-b-0"}
    clx! {CardContent, div, "pt-4"}
    clx! {CardFooter, div, "mt-4", "flex items-center justify-end"}
}

pub use components::*;
```
