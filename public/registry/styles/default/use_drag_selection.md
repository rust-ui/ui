---
title: "Use Drag Selection"
name: "use_drag_selection"
cargo_dependencies: []
registry_dependencies: ["data_grid"]
type: "components:hooks/"
path: "hooks/use_drag_selection.rs"
---

# Use Drag Selection

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_drag_selection
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::data_grid::DataGridColumn;

/// Return type for the drag selection hook
#[derive(Clone, Copy)]
pub struct UseDragSelection<C: DataGridColumn> {
    /// Whether a drag is currently in progress (mouse held down)
    is_dragging_signal: RwSignal<bool>,
    /// Start cell of the drag selection
    drag_start_signal: RwSignal<Option<(usize, C)>>,
    /// End cell of the drag selection
    drag_end_signal: RwSignal<Option<(usize, C)>>,
}

impl<C: DataGridColumn> UseDragSelection<C> {
    /// Check if a cell is within the drag selection range (bounding box of start and end).
    /// Returns false if start == end (single cell click, not a real drag).
    pub fn is_cell_in_range(&self, row_idx: usize, col: C) -> bool {
        let Some((start_row, start_col)) = self.drag_start_signal.get() else {
            return false;
        };
        let Some((end_row, end_col)) = self.drag_end_signal.get() else {
            return false;
        };

        // Only show range if it's an actual drag (start != end)
        if start_row == end_row && start_col == end_col {
            return false;
        }

        let min_row = start_row.min(end_row);
        let max_row = start_row.max(end_row);
        let start_col_idx = start_col.colindex();
        let end_col_idx = end_col.colindex();
        let min_col = start_col_idx.min(end_col_idx);
        let max_col = start_col_idx.max(end_col_idx);
        let col_idx = col.colindex();

        row_idx >= min_row && row_idx <= max_row && col_idx >= min_col && col_idx <= max_col
    }

    /// Returns true if there's an active multi-cell selection (start != end).
    pub fn has_selection(&self) -> bool {
        let Some((start_row, start_col)) = self.drag_start_signal.get() else {
            return false;
        };
        let Some((end_row, end_col)) = self.drag_end_signal.get() else {
            return false;
        };
        start_row != end_row || start_col != end_col
    }

    /// Returns the selection bounds as (min_row, max_row, min_col_idx, max_col_idx).
    /// Returns None if no selection or single cell selection.
    /// Uses `get_untracked` since this is called from event handlers, not reactive contexts.
    pub fn get_selection_bounds(&self) -> Option<(usize, usize, i32, i32)> {
        let (start_row, start_col) = self.drag_start_signal.get_untracked()?;
        let (end_row, end_col) = self.drag_end_signal.get_untracked()?;

        // No bounds for single cell
        if start_row == end_row && start_col == end_col {
            return None;
        }

        let min_row = start_row.min(end_row);
        let max_row = start_row.max(end_row);
        let min_col = start_col.colindex().min(end_col.colindex());
        let max_col = start_col.colindex().max(end_col.colindex());

        Some((min_row, max_row, min_col, max_col))
    }

    /// Start a new drag selection from the given cell. Called on mousedown.
    pub fn start_drag(&self, row_idx: usize, col: C) {
        self.is_dragging_signal.set(true);
        self.drag_start_signal.set(Some((row_idx, col)));
        self.drag_end_signal.set(Some((row_idx, col)));
    }

    /// Update the drag end point while dragging. Called on mouseenter.
    /// Only updates if a drag is in progress.
    /// Uses `get_untracked` since this is called from event handlers, not reactive contexts.
    pub fn update_drag(&self, row_idx: usize, col: C) {
        if self.is_dragging_signal.get_untracked() {
            self.drag_end_signal.set(Some((row_idx, col)));
        }
    }

    /// Stop the drag operation (but keep the selection). Called on mouseup/mouseleave.
    pub fn stop_dragging(&self) {
        self.is_dragging_signal.set(false);
    }

    /// Clear the drag selection entirely. Called on single click.
    pub fn clear_selection(&self) {
        self.drag_start_signal.set(None);
        self.drag_end_signal.set(None);
    }

    /// Handle right-click context menu logic.
    /// Returns true if selection was cleared (click was outside selection).
    /// Returns false if selection was preserved (click was inside selection).
    pub fn handle_contextmenu(&self, row_idx: usize, col: C) -> bool {
        let in_selection = self.is_cell_in_range(row_idx, col);
        if !in_selection {
            self.clear_selection();
            true
        } else {
            false
        }
    }

    /// Collect values from selected cells as a tab/newline separated string.
    /// Returns None if no multi-cell selection exists.
    ///
    /// Format: columns separated by tabs, rows separated by CRLF (Excel-compatible).
    pub fn collect_selection_values<R, F>(&self, rows: &[R], columns: &[(C, i32)], get_value: F) -> Option<String>
    where
        F: Fn(&R, C) -> String,
    {
        let (min_row, max_row, min_col, max_col) = self.get_selection_bounds()?;

        let mut result = Vec::new();
        for row_idx in min_row..=max_row {
            if let Some(row) = rows.get(row_idx) {
                let row_values: Vec<String> = columns
                    .iter()
                    .filter(|(col, _)| {
                        let col_idx = col.colindex();
                        col_idx >= min_col && col_idx <= max_col
                    })
                    .map(|(col, _)| get_value(row, *col))
                    .collect();
                result.push(row_values.join("\t"));
            }
        }

        Some(result.join("\r\n"))
    }
}

/// Hook for managing drag selection state in a data grid.
///
/// Provides methods to handle drag selection lifecycle:
/// - `start_drag()` - on mousedown
/// - `update_drag()` - on mouseenter
/// - `stop_dragging()` - on mouseup/mouseleave
/// - `clear_selection()` - on click
/// - `handle_contextmenu()` - on right-click
/// - `is_cell_in_range()` - check if cell is in selection
/// - `has_selection()` - check if multi-cell selection exists
pub fn use_drag_selection<C: DataGridColumn>() -> UseDragSelection<C> {
    let is_dragging_signal = RwSignal::new(false);
    let drag_start_signal: RwSignal<Option<(usize, C)>> = RwSignal::new(None);
    let drag_end_signal: RwSignal<Option<(usize, C)>> = RwSignal::new(None);

    UseDragSelection { is_dragging_signal, drag_start_signal, drag_end_signal }
}
```
