---
title: "Use Handle Day Click"
name: "use_handle_day_click"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks/"
path: "hooks/use_handle_day_click.rs"
---

# Use Handle Day Click

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_handle_day_click
```

## Component Code

```rust
use leptos::prelude::*;
use time::Date;

use crate::utils::query::QueryUtils;

/// Hook for managing date range selection with URL synchronization
///
/// Returns a tuple of (start_date, end_date, handle_day_click) where:
/// - `start_date`: RwSignal<Date> for the start date
/// - `end_date`: RwSignal<Date> for the end date
/// - `handle_day_click`: Function that takes a day number and updates the closest date
pub fn use_handle_day_click(
    initial_start: Date,
    initial_end: Date,
) -> (RwSignal<Date>, RwSignal<Date>, impl Fn(u8) + Clone) {
    let start_date_signal = RwSignal::new(initial_start);
    let end_date_signal = RwSignal::new(initial_end);

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }

        let year = start_date_signal.get().year();
        let month = start_date_signal.get().month();

        // Create new date for the selected day
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        // Determine which date to update based on proximity
        let current_start = start_date_signal.get().day();
        let current_end = end_date_signal.get().day();

        if current_start.abs_diff(day) <= current_end.abs_diff(day) {
            start_date_signal.set(new_date);
        } else {
            end_date_signal.set(new_date);
        }

        // Update URL with new dates
        QueryUtils::update_dates_url(Some(start_date_signal.get()), Some(end_date_signal.get()));
    };

    (start_date_signal, end_date_signal, handle_day_click)
}
```
