---
title: "Demo Date Picker Week Numbers"
name: "demo_date_picker_week_numbers"
cargo_dependencies: []
registry_dependencies: ["date_picker", "date_picker_state"]
type: "components:demos"
path: "demos/demo_date_picker_week_numbers.rs"
---

# Demo Date Picker Week Numbers

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_date_picker_week_numbers
```

## Component Code

```rust
use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;
use time::{Date, Month};

use crate::components::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay, DatePickerWeekNumberCell, DatePickerWeekNumberHeader,
};
use crate::components::ui::date_picker_state::{DatePickerDay, DatePickerState};
use crate::utils::date::DateUtils;

#[component]
pub fn DemoDatePickerWeekNumbers() -> impl IntoView {
    let Some(default_start) = Date::from_calendar_date(2025, Month::May, 5).ok() else {
        return view! { <p>"Invalid default start date"</p> }.into_any();
    };
    let Some(default_end) = Date::from_calendar_date(2025, Month::May, 14).ok() else {
        return view! { <p>"Invalid default end date"</p> }.into_any();
    };

    let start_date_signal = RwSignal::new(default_start);
    let end_date_signal = RwSignal::new(default_end);
    let display_date_signal = RwSignal::new(default_start);

    let go_to_previous_month = move |_| {
        let current = display_date_signal.get();
        let (prev_month, prev_year) = DateUtils::prev_month_year(current.month(), current.year());
        if let Ok(new_date) = Date::from_calendar_date(prev_year, prev_month, 1) {
            display_date_signal.set(new_date);
        }
    };

    let go_to_next_month = move |_| {
        let current = display_date_signal.get();
        let (next_month, next_year) = DateUtils::next_month_year(current.month(), current.year());
        if let Ok(new_date) = Date::from_calendar_date(next_year, next_month, 1) {
            display_date_signal.set(new_date);
        }
    };

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }
        let year = display_date_signal.get().year();
        let month = display_date_signal.get().month();
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        let current_start = start_date_signal.get();
        let current_end = end_date_signal.get();
        let days_from_start = (new_date - current_start).whole_days().abs();
        let days_from_end = (new_date - current_end).whole_days().abs();

        if days_from_start <= days_from_end {
            start_date_signal.set(new_date);
        } else {
            end_date_signal.set(new_date);
        }

        if start_date_signal.get() > end_date_signal.get() {
            let temp = start_date_signal.get();
            start_date_signal.set(end_date_signal.get());
            end_date_signal.set(temp);
        }
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
                        <DatePickerWeekNumberHeader attr:aria-label="Week">"#"</DatePickerWeekNumberHeader>
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
                                    let week_num = week
                                        .iter()
                                        .find(|d| !d.disabled)
                                        .and_then(|d| Date::from_calendar_date(year, month, d.day).ok())
                                        .map(|d| d.iso_week());

                                    view! {
                                        <DatePickerRow>
                                            <DatePickerWeekNumberCell>
                                                {week_num.map(|n| n.to_string()).unwrap_or_default()}
                                            </DatePickerWeekNumberCell>
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
