---
title: "Demo Date Picker"
name: "demo_date_picker"
cargo_dependencies: []
registry_dependencies: ["date_picker", "date_picker_state"]
type: "components:demos"
path: "demos/demo_date_picker.rs"
---

# Demo Date Picker

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_date_picker
```

## Component Code

```rust
use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;
use time::{Date, Month};

use crate::components::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::components::ui::date_picker_state::{DatePickerDay, DatePickerState};
use crate::utils::query::QueryUtils;

#[component]
pub fn DemoDatePicker() -> impl IntoView {
    // Initialize dates from URL or use defaults
    let Some(default_start) = Date::from_calendar_date(2025, Month::May, 5).ok() else {
        return view! { <p>"Invalid default start date"</p> }.into_any();
    };
    let Some(default_end) = Date::from_calendar_date(2025, Month::May, 14).ok() else {
        return view! { <p>"Invalid default end date"</p> }.into_any();
    };

    let picker_state = DatePickerState::from_url_or_default(default_start, default_end);

    // Extract start and end dates as RwSignal
    let start_date_signal = RwSignal::new(picker_state.get_untracked().start_date);
    let end_date_signal = RwSignal::new(picker_state.get_untracked().end_date);

    // Track which month is currently displayed (independent from selected dates)
    let display_date_signal = RwSignal::new(picker_state.get_untracked().start_date);

    // Sync local signals with picker_state when URL changes
    Effect::new(move |_| {
        let state = picker_state.get();
        start_date_signal.set(state.start_date);
        end_date_signal.set(state.end_date);
        // Don't reset display_date_signal here - let user navigation control it
    });

    // Navigation: Go to previous month
    let go_to_previous_month = move |_| {
        let current = display_date_signal.get();
        let Some(new_date) = (if current.month() == Month::January {
            Date::from_calendar_date(current.year() - 1, Month::December, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().previous(), 1)
        })
        .ok() else {
            return;
        };
        display_date_signal.set(new_date);
    };

    // Navigation: Go to next month
    let go_to_next_month = move |_| {
        let current = display_date_signal.get();
        let Some(new_date) = (if current.month() == Month::December {
            Date::from_calendar_date(current.year() + 1, Month::January, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().next(), 1)
        })
        .ok() else {
            return;
        };
        display_date_signal.set(new_date);
    };

    // Handle day click
    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }

        let year = display_date_signal.get().year();
        let month = display_date_signal.get().month();
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        // Determine which date to update based on proximity (using full dates)
        let current_start = start_date_signal.get();
        let current_end = end_date_signal.get();

        let days_from_start = (new_date - current_start).whole_days().abs();
        let days_from_end = (new_date - current_end).whole_days().abs();

        if days_from_start <= days_from_end {
            start_date_signal.set(new_date);
        } else {
            end_date_signal.set(new_date);
        }

        // Ensure start_date_signal <= end_date_signal by swapping if needed
        if start_date_signal.get() > end_date_signal.get() {
            let temp = start_date_signal.get();
            start_date_signal.set(end_date_signal.get());
            end_date_signal.set(temp);
        }

        QueryUtils::update_dates_url(Some(start_date_signal.get()), Some(end_date_signal.get()));
    };

    view! {
        <DatePicker>
            <DatePickerHeader>
                <DatePickerNavButton
                    class="justify-self-start"
                    attr:title="previous-month"
                    attr:aria-label="Go to previous month"
                    on:click=go_to_previous_month
                >
                    <ChevronLeft />
                </DatePickerNavButton>
                <DatePickerTitle attr:role="presentation">
                    {move || display_date_signal.get().month().to_string()} {move || display_date_signal.get().year()}
                </DatePickerTitle>
                <DatePickerNavButton
                    class="justify-self-end"
                    attr:title="next-month"
                    attr:aria-label="Go to next month"
                    on:click=go_to_next_month
                >
                    <ChevronRight />
                </DatePickerNavButton>
            </DatePickerHeader>

            <table class="space-y-1 w-full border-collapse" role="grid">
                <thead>
                    <tr class="flex">
                        <DatePickerWeekDay attr:aria-label="Monday">Mo</DatePickerWeekDay>
                        <DatePickerWeekDay attr:aria-label="Tuesday">Tu</DatePickerWeekDay>
                        <DatePickerWeekDay attr:aria-label="Wednesday">We</DatePickerWeekDay>
                        <DatePickerWeekDay attr:aria-label="Thursday">Th</DatePickerWeekDay>
                        <DatePickerWeekDay attr:aria-label="Friday">Fr</DatePickerWeekDay>
                        <DatePickerWeekDay attr:aria-label="Saturday">Sa</DatePickerWeekDay>
                        <DatePickerWeekDay attr:aria-label="Sunday">Su</DatePickerWeekDay>
                    </tr>
                </thead>

                <tbody>
                    {move || {
                        let year = display_date_signal.get().year();
                        let month = display_date_signal.get().month();
                        let days = DatePickerState::get_calendar_days(year, month);
                        let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

                        view! {
                            <For
                                each=move || weeks.clone()
                                key=|week| week.first().map(|d| d.day).unwrap_or(0)
                                children=move |week| {
                                    let year = year;
                                    let month = month;
                                    view! {
                                        <DatePickerRow>
                                            <For
                                                each=move || week.clone()
                                                key=|DatePickerDay { day, disabled }| format!("{day}-{disabled}")
                                                children=move |DatePickerDay { day, disabled }| {
                                                    view! {
                                                        <DatePickerCell
                                                            day=day
                                                            year=year
                                                            month=month
                                                            disabled=disabled
                                                            start_date=start_date_signal
                                                            end_date=end_date_signal
                                                            on_click=handle_day_click
                                                        />
                                                    }
                                                }
                                            />
                                        </DatePickerRow>
                                    }
                                }
                            />
                        }
                    }}
                </tbody>
            </table>
        </DatePicker>
    }
    .into_any()
}
```
