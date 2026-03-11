---
title: "Date"
name: "date"
cargo_dependencies: []
registry_dependencies: []
type: "components:utils"
path: "utils/date.rs"
---

# Date

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add date
```

## Component Code

```rust
use time::{Date, Month};

pub struct DateUtils;

impl DateUtils {
    /// Parse date string from URL (YYYY-MM-DD format)
    pub fn parse_from_url(date_str: Option<String>) -> Option<Date> {
        date_str.and_then(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() == 3 {
                let year = parts.first()?.parse::<i32>().ok()?;
                let month = Month::try_from(parts.get(1)?.parse::<u8>().ok()?).ok()?;
                let day = parts.get(2)?.parse::<u8>().ok()?;
                Date::from_calendar_date(year, month, day).ok()
            } else {
                None
            }
        })
    }

    /// Get the next month and year, handling December -> January rollover
    pub fn next_month_year(month: Month, year: i32) -> (Month, i32) {
        if month == Month::December { (Month::January, year + 1) } else { (month.next(), year) }
    }

    /// Get the previous month and year, handling January -> December rollover
    pub fn prev_month_year(month: Month, year: i32) -> (Month, i32) {
        if month == Month::January { (Month::December, year - 1) } else { (month.previous(), year) }
    }
}
```
