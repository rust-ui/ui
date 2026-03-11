---
title: "Use Cell Selection"
name: "use_cell_selection"
cargo_dependencies: []
registry_dependencies: ["data_grid"]
type: "components:hooks/"
path: "hooks/use_cell_selection.rs"
---

# Use Cell Selection

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_cell_selection
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::data_grid::DataGridColumn;

/// Return type for the cell selection hook.
/// Manages active cell (click) and context menu cell (right-click) state.
#[derive(Clone, Copy)]
pub struct UseCellSelection<C: DataGridColumn> {
    /// The currently active/focused cell (left-clicked)
    active_cell_signal: RwSignal<Option<(usize, C)>>,
    /// The cell that triggered the context menu (right-clicked)
    context_menu_cell_signal: RwSignal<Option<(usize, C)>>,
    /// Prevents race condition when right-clicking multiple cells consecutively.
    /// Without this, `on_close` fires after `on_contextmenu`, clearing newly set values.
    context_menu_reopening_signal: RwSignal<bool>,
}

impl<C: DataGridColumn> UseCellSelection<C> {
    /// Check if a specific cell is the active cell.
    pub fn is_active(&self, row_idx: usize, col: C) -> bool {
        self.active_cell_signal.get() == Some((row_idx, col))
    }

    /// Check if a specific cell is the context menu cell.
    pub fn is_context_menu(&self, row_idx: usize, col: C) -> bool {
        self.context_menu_cell_signal.get() == Some((row_idx, col))
    }

    /// Set the active cell (typically on left-click).
    pub fn set_active(&self, row_idx: usize, col: C) {
        self.active_cell_signal.set(Some((row_idx, col)));
    }

    /// Clear the active cell.
    pub fn clear_active(&self) {
        self.active_cell_signal.set(None);
    }

    /// Set the context menu cell (typically on right-click).
    pub fn set_context_menu(&self, row_idx: usize, col: C) {
        self.context_menu_cell_signal.set(Some((row_idx, col)));
    }

    /// Clear the context menu cell.
    pub fn clear_context_menu(&self) {
        self.context_menu_cell_signal.set(None);
    }

    /// Clear all cell selections (active and context menu).
    pub fn clear_all(&self) {
        self.active_cell_signal.set(None);
        self.context_menu_cell_signal.set(None);
    }

    /// Handle left-click on a cell.
    /// Sets the active cell and clears context menu highlight.
    pub fn handle_click(&self, row_idx: usize, col: C) {
        self.active_cell_signal.set(Some((row_idx, col)));
        self.context_menu_cell_signal.set(None);
    }

    /// Signal that a context menu is about to open (call before handle_contextmenu).
    /// This prevents the race condition where on_close clears newly set values.
    pub fn start_contextmenu(&self) {
        self.context_menu_reopening_signal.set(true);
    }

    /// Handle right-click on a cell.
    /// Sets both active and context menu cell.
    pub fn handle_contextmenu(&self, row_idx: usize, col: C) {
        self.start_contextmenu();
        self.active_cell_signal.set(Some((row_idx, col)));
        self.context_menu_cell_signal.set(Some((row_idx, col)));
    }

    /// Handle context menu close event.
    /// Only clears if not reopening on another cell.
    pub fn handle_contextmenu_close(&self) {
        if self.context_menu_reopening_signal.get() {
            self.context_menu_reopening_signal.set(false);
        } else {
            self.active_cell_signal.set(None);
            self.context_menu_cell_signal.set(None);
        }
    }
}

/// Hook for managing cell selection state in a data grid.
///
/// Provides methods to handle:
/// - Active cell (left-click) with ring highlight
/// - Context menu cell (right-click) with background highlight
/// - Race condition prevention for consecutive right-clicks
pub fn use_cell_selection<C: DataGridColumn>() -> UseCellSelection<C> {
    let active_cell_signal = RwSignal::new(None);
    let context_menu_cell_signal = RwSignal::new(None);
    let context_menu_reopening_signal = RwSignal::new(false);

    UseCellSelection { active_cell_signal, context_menu_cell_signal, context_menu_reopening_signal }
}
```
