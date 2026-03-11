---
title: "Bento Grid"
name: "bento_grid"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/bento_grid.rs"
---

# Bento Grid

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add bento_grid
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {BentoGrid, div, "grid gap-2 md:grid-cols-4"}
    clx! {BentoGrid6, div, "grid gap-2 sm:grid-cols-2 md:grid-cols-4"}
    clx! {BentoRow, div, "p-1 min-h-32 rounded-lg"}
    clx! {BentoCell, div, "text-xl rounded-lg size-full center bg-zinc-200 dark:bg-zinc-700"}
}

pub use components::*;
```
