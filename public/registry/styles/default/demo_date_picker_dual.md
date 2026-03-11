---
title: "Demo Date Picker Dual"
name: "demo_date_picker_dual"
cargo_dependencies: []
registry_dependencies: ["date_picker", "date_picker_state"]
type: "components:demos"
path: "demos/demo_date_picker_dual.rs"
---

# Demo Date Picker Dual

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_date_picker_dual
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
pub fn DemoDatePickerDual() -> impl IntoView {
    // Initialize dates from URL or use defaults
    let Some(default_start) = Date::from_calendar_date(2025, Month::May, 5).ok() else {
        return view! { <p>"Invalid default start date"</p> }.into_any();
    };
    let Some(default_end) = Date::from_calendar_date(2025, Month::June, 8).ok() else {
        return view! { <p>"Invalid default end date"</p> }.into_any();
    };

    let picker_state = DatePickerState::from_url_or_default(default_start, default_end);

    // Extract start and end dates as RwSignal
    let start_date_signal = RwSignal::new(picker_state.get_untracked().start_date);
    let end_date_signal = RwSignal::new(picker_state.get_untracked().end_date);

    // Track the left calendar month (right will be left + 1)
    let left_display_date_signal = RwSignal::new(picker_state.get_untracked().start_date);

    // Sync local signals with picker_state when URL changes
    Effect::new(move |_| {
        let state = picker_state.get();
        start_date_signal.set(state.start_date);
        end_date_signal.set(state.end_date);
    });

    // Navigation: Go to previous 2 months
    let go_to_previous = move |_| {
        let current = left_display_date_signal.get();
        let Some(new_date) = (if current.month() == Month::January {
            Date::from_calendar_date(current.year() - 1, Month::November, 1)
        } else if current.month() == Month::February {
            Date::from_calendar_date(current.year() - 1, Month::December, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().previous().previous(), 1)
        })
        .ok() else {
            return;
        };
        left_display_date_signal.set(new_date);
    };

    // Navigation: Go to next 2 months
    let go_to_next = move |_| {
        let current = left_display_date_signal.get();
        let Some(new_date) = (if current.month() == Month::November {
            Date::from_calendar_date(current.year() + 1, Month::January, 1)
        } else if current.month() == Month::December {
            Date::from_calendar_date(current.year() + 1, Month::February, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().next().next(), 1)
        })
        .ok() else {
            return;
        };
        left_display_date_signal.set(new_date);
    };

    // Handle day click - works for both calendars
    let handle_day_click = move |year: i32, month: Month, day: u8| {
        if day == 0 {
            return;
        }

        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        // If clicking before or at start date, set as new start
        // Otherwise set as end date
        if new_date <= start_date_signal.get() {
            start_date_signal.set(new_date);
        } else {
            end_date_signal.set(new_date);
        }

        QueryUtils::update_dates_url(Some(start_date_signal.get()), Some(end_date_signal.get()));
    };

    // Helper to render a single calendar
    let render_calendar = move |is_left: bool| {
        let display_date = if is_left {
            left_display_date_signal.get()
        } else {
            // Right calendar is left + 1 month
            let left = left_display_date_signal.get();
            (if left.month() == Month::December {
                Date::from_calendar_date(left.year() + 1, Month::January, 1)
            } else {
                Date::from_calendar_date(left.year(), left.month().next(), 1)
            })
            .unwrap_or(left)
        };

        let year = display_date.year();
        let month = display_date.month();

        view! {
            <DatePicker>
                <DatePickerHeader>
                    {move || {
                        if is_left {
                            view! {
                                <DatePickerNavButton
                                    class="justify-self-start"
                                    attr:title="previous-months"
                                    attr:aria-label="Go to previous months"
                                    attr:disabled=true
                                    on:click=go_to_previous
                                >
                                    <ChevronLeft />
                                </DatePickerNavButton>
                            }
                                .into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }} <DatePickerTitle attr:role="presentation">{month.to_string()} {year}</DatePickerTitle>
                    {move || {
                        if !is_left {
                            view! {
                                <DatePickerNavButton
                                    class="justify-self-end"
                                    attr:title="next-months"
                                    attr:aria-label="Go to next months"
                                    attr:disabled=true
                                    on:click=go_to_next
                                >
                                    <ChevronRight />
                                </DatePickerNavButton>
                            }
                                .into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                </DatePickerHeader>

                <table class="space-y-1 w-full border-collapse" role="grid">
                    <thead>
                        <tr class="flex">
                            <DatePickerWeekDay attr:aria-label="Sunday">Su</DatePickerWeekDay>
                            <DatePickerWeekDay attr:aria-label="Monday">Mo</DatePickerWeekDay>
                            <DatePickerWeekDay attr:aria-label="Tuesday">Tu</DatePickerWeekDay>
                            <DatePickerWeekDay attr:aria-label="Wednesday">We</DatePickerWeekDay>
                            <DatePickerWeekDay attr:aria-label="Thursday">Th</DatePickerWeekDay>
                            <DatePickerWeekDay attr:aria-label="Friday">Fr</DatePickerWeekDay>
                            <DatePickerWeekDay attr:aria-label="Saturday">Sa</DatePickerWeekDay>
                        </tr>
                    </thead>

                    <tbody>
                        {move || {
                            let days = DatePickerState::get_calendar_days(year, month);
                            let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

                            view! {
                                <For
                                    each=move || weeks.clone()
                                    key=|week| week.first().map(|d| d.day).unwrap_or(0)
                                    children=move |week| {
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
                                                                on_click=move |d| handle_day_click(year, month, d)
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
    };

    view! { <div class="flex gap-4">{render_calendar(true)} {render_calendar(false)}</div> }.into_any()
}
```
