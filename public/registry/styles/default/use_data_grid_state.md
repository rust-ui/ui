---
title: "Use Data Grid State"
name: "use_data_grid_state"
cargo_dependencies: []
registry_dependencies: ["data_grid", "use_cell_selection", "use_click_outside", "use_drag_selection"]
type: "components:hooks"
path: "hooks/use_data_grid_state.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Data Grid State

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_data_grid_state
```

## Component Code

```rust
//! Shared state and hooks for data grid components.
//!
//! Consolidates the common setup used across data grids:
//! - Cell selection (active cell, context menu cell)
//! - Drag selection (range selection)
//! - Click outside detection

use dioxus::prelude::*;

use crate::components::hooks::use_cell_selection::{UseCellSelection, use_cell_selection};
use crate::components::hooks::use_click_outside::use_click_outside;
use crate::components::hooks::use_drag_selection::{UseDragSelection, use_drag_selection};
use crate::components::ui::data_grid::DataGridColumn;

/// State returned by `use_data_grid_state` hook.
pub struct DataGridState<C: DataGridColumn> {
    /// Cell selection hook (active cell + context menu cell).
    pub cell_selection: UseCellSelection<C>,
    /// Drag selection hook (range selection).
    pub drag_selection: UseDragSelection<C>,
    /// Value to copy (set when right-clicking a cell).
    pub copy_value_signal: Signal<String>,
    /// Element signal for click-outside detection (set from onmounted).
    pub grid_wrapper_element: Signal<Option<web_sys::Element>>,
}

/// Hook that sets up common data grid state and behaviors.
///
/// Returns a `DataGridState` containing:
/// - Cell selection for active/context menu cells
/// - Drag selection for range selection
/// - Grid wrapper element with click-outside handling
///
/// # Example
/// ```ignore
/// let grid_state = use_data_grid_state::<Column>();
/// // Use grid_state.cell_selection, grid_state.drag_selection, etc.
/// ```
pub fn use_data_grid_state<C: DataGridColumn + 'static>() -> DataGridState<C> {
    let cell_selection = use_cell_selection::<C>();
    let drag_selection = use_drag_selection::<C>();

    let copy_value_signal: Signal<String> = use_signal(String::new);

    let grid_wrapper_element: Signal<Option<web_sys::Element>> = use_signal(|| None);

    // Clear all highlights when clicking outside the grid
    use_click_outside(grid_wrapper_element.into(), move || {
        cell_selection.clone().clear_all();
        drag_selection.clone().clear_selection();
    });

    DataGridState { cell_selection, drag_selection, copy_value_signal, grid_wrapper_element }
}
```
