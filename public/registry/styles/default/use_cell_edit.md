---
title: "Use Cell Edit"
name: "use_cell_edit"
cargo_dependencies: []
registry_dependencies: ["data_grid"]
type: "components:hooks"
path: "hooks/use_cell_edit.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Cell Edit

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_cell_edit
```

## Component Code

```rust
use dioxus::prelude::*;

use crate::components::ui::data_grid::DataGridColumn;

/// Context for sharing cell edit state across components.
#[derive(Clone, Copy)]
pub struct CellEditContext<C: DataGridColumn + 'static> {
    /// The cell currently being edited (row_idx, column)
    editing_cell: Signal<Option<(usize, C)>>,
    /// The current value in the edit input
    pub edit_value: Signal<String>,
}

impl<C: DataGridColumn> CellEditContext<C> {
    /// Check if a specific cell is currently being edited.
    pub fn is_editing(&self, row_idx: usize, col: C) -> bool {
        *self.editing_cell.peek() == Some((row_idx, col))
    }

    /// Start editing a cell with an initial value.
    pub fn start_edit(&mut self, row_idx: usize, col: C, initial_value: String) {
        self.editing_cell.set(Some((row_idx, col)));
        self.edit_value.set(initial_value);
    }

    /// Cancel editing and discard changes.
    pub fn cancel_edit(&mut self) {
        self.editing_cell.set(None);
        self.edit_value.set(String::new());
    }

    /// Finish editing and return the final value.
    pub fn finish_edit(&mut self) -> Option<(usize, C, String)> {
        let cell = (*self.editing_cell.peek())?;
        let value = self.edit_value.peek().clone();
        self.editing_cell.set(None);
        self.edit_value.set(String::new());
        Some((cell.0, cell.1, value))
    }
}

/// Create and provide cell edit context. Call once at the grid level.
pub fn use_cell_edit<C: DataGridColumn + 'static>() -> CellEditContext<C> {
    let ctx = CellEditContext { editing_cell: use_signal(|| None), edit_value: use_signal(String::new) };
    provide_context(ctx);
    ctx
}
```
