---
title: "Demo Date Picker Booked"
name: "demo_date_picker_booked"
cargo_dependencies: []
registry_dependencies: ["card", "date_picker", "date_picker_state"]
type: "components:demos"
path: "demos/demo_date_picker_booked.rs"
---

# Demo Date Picker Booked

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_date_picker_booked
```

## Component Code

```rust
use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;
use time::{Date, Month, OffsetDateTime};

use crate::components::ui::card::{Card, CardContent};
use crate::components::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::components::ui::date_picker_state::{DatePickerDay, DatePickerState};

/// Days 12–26 of the booked month are marked as booked (struck through, not selectable).
fn is_booked_day(year: i32, month: Month, day: u8, booked_year: i32, booked_month: Month) -> bool {
    day > 0 && year == booked_year && month == booked_month && (12..=26).contains(&day)
}

#[component]
pub fn DemoDatePickerBooked() -> impl IntoView {
    let current_year = OffsetDateTime::now_utc().date().year();

    let booked_year = current_year;
    let booked_month = Month::February;

    let initial_selected =
        Date::from_calendar_date(current_year, Month::February, 3).unwrap_or_else(|_| OffsetDateTime::now_utc().date());
    let initial_display =
        Date::from_calendar_date(current_year, Month::February, 1).unwrap_or_else(|_| OffsetDateTime::now_utc().date());

    let selected_date = RwSignal::new(initial_selected);
    let display_date = RwSignal::new(initial_display);

    let go_to_previous_month = move |_| {
        let current = display_date.get();
        let new_date = if current.month() == Month::January {
            Date::from_calendar_date(current.year() - 1, Month::December, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().previous(), 1)
        };
        if let Ok(d) = new_date {
            display_date.set(d);
        }
    };

    let go_to_next_month = move |_| {
        let current = display_date.get();
        let new_date = if current.month() == Month::December {
            Date::from_calendar_date(current.year() + 1, Month::January, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().next(), 1)
        };
        if let Ok(d) = new_date {
            display_date.set(d);
        }
    };

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }
        let year = display_date.get().year();
        let month = display_date.get().month();
        if let Ok(new_date) = Date::from_calendar_date(year, month, day) {
            selected_date.set(new_date);
        }
    };

    view! {
        <Card class="w-fit">
            <CardContent class="p-0">
                <DatePicker>
                    <DatePickerHeader>
                        <DatePickerNavButton
                            class="justify-self-start"
                            attr:aria-label="Go to previous month"
                            on:click=go_to_previous_month
                        >
                            <ChevronLeft />
                        </DatePickerNavButton>
                        <DatePickerTitle attr:role="presentation">
                            {move || display_date.get().month().to_string()} " " {move || display_date.get().year()}
                        </DatePickerTitle>
                        <DatePickerNavButton
                            class="justify-self-end"
                            attr:aria-label="Go to next month"
                            on:click=go_to_next_month
                        >
                            <ChevronRight />
                        </DatePickerNavButton>
                    </DatePickerHeader>

                    <table class="space-y-1 w-full border-collapse" role="grid">
                        <thead>
                            <tr class="flex">
                                <DatePickerWeekDay attr:aria-label="Monday">"Mo"</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Tuesday">"Tu"</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Wednesday">"We"</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Thursday">"Th"</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Friday">"Fr"</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Saturday">"Sa"</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Sunday">"Su"</DatePickerWeekDay>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let year = display_date.get().year();
                                let month = display_date.get().month();
                                let days = DatePickerState::get_calendar_days(year, month);
                                let weeks: Vec<Vec<DatePickerDay>> = days
                                    .chunks(7)
                                    .map(|chunk| chunk.to_vec())
                                    .collect();
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
                                                        key=|DatePickerDay { day, disabled }| {
                                                            format!("{day}-{disabled}")
                                                        }
                                                        children=move |DatePickerDay { day, disabled }| {
                                                            let booked = !disabled
                                                                && is_booked_day(
                                                                    year,
                                                                    month,
                                                                    day,
                                                                    booked_year,
                                                                    booked_month,
                                                                );
                                                            let cell_class = if booked {
                                                                "line-through aria-disabled:opacity-100"
                                                            } else {
                                                                ""
                                                            };
                                                            view! {
                                                                <DatePickerCell
                                                                    day=day
                                                                    year=year
                                                                    month=month
                                                                    disabled=disabled || booked
                                                                    start_date=selected_date
                                                                    end_date=selected_date
                                                                    on_click=handle_day_click
                                                                    class=cell_class
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
            </CardContent>
        </Card>
    }
}
```
