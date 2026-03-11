---
title: "Date Picker State"
name: "date_picker_state"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/date_picker_state.rs"
---

# Date Picker State

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add date_picker_state
```

## Component Code

```rust
use leptos::prelude::*;
use time::{Date, Month};

use crate::utils::date::DateUtils;
use crate::utils::query::{QUERY, QueryUtils};

#[derive(Debug, Clone, Copy)]
pub struct DatePickerDay {
    pub day: u8,
    pub disabled: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub struct DatePickerState {
    pub start_date: Date,
    pub end_date: Date,
}

impl DatePickerState {
    pub fn new(start_date: Date, end_date: Date) -> Self {
        Self { start_date, end_date }
    }

    /// Create a new state from URL parameters with fallback defaults
    pub fn from_url_or_default(default_start: Date, default_end: Date) -> RwSignal<Self> {
        let start_date_query = QueryUtils::extract(QUERY::START_DATE.to_string());
        let end_date_query = QueryUtils::extract(QUERY::END_DATE.to_string());

        let state_signal = RwSignal::new(Self::new(default_start, default_end));

        // Effect to sync from URL to state
        Effect::new(move |_| {
            let start = start_date_query.get();
            let end = end_date_query.get();

            let start_date_parsed = DateUtils::parse_from_url(start);
            let end_date_parsed = DateUtils::parse_from_url(end);

            // Check if we have both dates from URL and if they're invalid
            let dates_from_url_invalid =
                if let (Some(s), Some(e)) = (start_date_parsed, end_date_parsed) { s > e } else { false };

            let (start_date, end_date) = if dates_from_url_invalid {
                // Invalid dates from URL - use defaults and clear query (in browser only)
                #[cfg(target_arch = "wasm32")]
                QueryUtils::update_dates_url(None, None);

                (default_start, default_end)
            } else {
                (start_date_parsed.unwrap_or(default_start), end_date_parsed.unwrap_or(default_end))
            };

            state_signal.set(Self::new(start_date, end_date));
        });

        state_signal
    }

    /// Gets calendar days for the given month, including leading/trailing days from adjacent months
    pub fn get_calendar_days(year: i32, month: Month) -> Vec<DatePickerDay> {
        let Some(first_day) = Date::from_calendar_date(year, month, 1).ok() else { return vec![] };
        let first_weekday = first_day.weekday().number_from_monday() as usize - 1;

        let (prev_month, prev_year) = DateUtils::prev_month_year(month, year);
        let days_in_prev_month = prev_month.length(prev_year);
        let days_in_month = month.length(year);

        let mut days = vec![];

        // Leading days from previous month
        for i in 0..first_weekday {
            let day = days_in_prev_month - (first_weekday as u8) + (i as u8) + 1;
            days.push(DatePickerDay { day, disabled: true });
        }

        // Days in current month
        for day in 1..=days_in_month {
            days.push(DatePickerDay { day, disabled: false });
        }

        // Trailing days from next month to fill the last week
        let trailing = (7 - days.len() % 7) % 7;
        for day in 1..=trailing as u8 {
            days.push(DatePickerDay { day, disabled: true });
        }

        days
    }
}
```
