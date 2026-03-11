---
title: "Use Column State"
name: "use_column_state"
cargo_dependencies: []
registry_dependencies: ["data_grid"]
type: "components:hooks/"
path: "hooks/use_column_state.rs"
---

# Use Column State

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_column_state
```

## Component Code

```rust
//! Shared column state for data grid components.
//!
//! Consolidates the common column setup used across data grids:
//! - Sort signals (per-column sort direction)
//! - Pinned columns (sticky columns)
//! - Visible columns (column visibility toggle)

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use leptos::prelude::*;
use strum::IntoEnumIterator;

use crate::components::ui::data_grid::{DataGridColumn, SortDirection};

/// State returned by `use_column_state` hook.
pub struct ColumnState<C: DataGridColumn> {
    /// Sort signals for each column.
    pub sort_signals: StoredValue<HashMap<C, RwSignal<SortDirection>>>,
    /// Which columns are pinned (sticky).
    pub pinned_columns_signal: RwSignal<HashSet<C>>,
    /// Which columns are visible.
    pub visible_columns_signal: RwSignal<HashSet<String>>,
}

/// Hook that sets up column state for data grids.
///
/// # Example
/// ```ignore
/// let ColumnState {
///     sort_signals,
///     pinned_columns_signal,
///     visible_columns_signal,
/// } = use_column_state::<Column>(PINNABLE_COLUMNS);
/// ```
pub fn use_column_state<C>(pinnable_columns: &[(C, i32)]) -> ColumnState<C>
where
    C: DataGridColumn + IntoEnumIterator + ToString + Hash + Eq + Copy + 'static,
{
    let sort_signals: HashMap<C, RwSignal<SortDirection>> =
        pinnable_columns.iter().map(|(col, _)| (*col, RwSignal::new(SortDirection::None))).collect();
    let sort_signals = StoredValue::new(sort_signals);

    let pinned_columns_signal = RwSignal::new(HashSet::<C>::new());

    let visible_columns_signal = RwSignal::new(C::iter().map(|c| c.to_string()).collect::<HashSet<String>>());

    ColumnState { sort_signals, pinned_columns_signal, visible_columns_signal }
}
```
