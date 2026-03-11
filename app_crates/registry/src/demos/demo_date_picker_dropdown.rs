use leptos::prelude::*;
use time::{Date, Month, OffsetDateTime};

use crate::ui::date_picker::{DatePicker, DatePickerCell, DatePickerHeader, DatePickerRow, DatePickerWeekDay};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

const MONTHS: [(u8, &str); 12] = [
    (1, "January"),
    (2, "February"),
    (3, "March"),
    (4, "April"),
    (5, "May"),
    (6, "June"),
    (7, "July"),
    (8, "August"),
    (9, "September"),
    (10, "October"),
    (11, "November"),
    (12, "December"),
];

#[component]
pub fn DemoDatePickerDropdown() -> impl IntoView {
    let today = OffsetDateTime::now_utc().date();
    let selected_date = RwSignal::new(today);
    let display_date = RwSignal::new(today);

    let years: Vec<i32> = (1924..=2030).collect();

    let handle_month_change = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev).parse::<u8>().unwrap_or(1);
        let month = Month::try_from(val).unwrap_or(Month::January);
        let current = display_date.get();
        if let Ok(new_date) = Date::from_calendar_date(current.year(), month, 1) {
            display_date.set(new_date);
        }
    };

    let handle_year_change = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev).parse::<i32>().unwrap_or(today.year());
        let current = display_date.get();
        if let Ok(new_date) = Date::from_calendar_date(val, current.month(), 1) {
            display_date.set(new_date);
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

    let select_class = "h-7 rounded-md border border-input bg-transparent px-2 text-sm focus:outline-none focus:ring-1 focus:ring-ring";

    view! {
        <DatePicker>
            <DatePickerHeader class="flex gap-2">
                <select class=format!("flex-1 {select_class}") on:change=handle_month_change>
                    {MONTHS
                        .iter()
                        .map(|(num, name)| {
                            let num = *num;
                            view! {
                                <option value=num selected=move || display_date.get().month() as u8 == num>
                                    {*name}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
                <select class=format!("w-[5.5rem] {select_class}") on:change=handle_year_change>
                    {years
                        .iter()
                        .map(|&year| {
                            view! {
                                <option value=year selected=move || display_date.get().year() == year>
                                    {year}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
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
                                                key=|DatePickerDay { day, disabled }| { format!("{day}-{disabled}") }
                                                children=move |DatePickerDay { day, disabled }| {
                                                    view! {
                                                        <DatePickerCell
                                                            day=day
                                                            year=year
                                                            month=month
                                                            disabled=disabled
                                                            start_date=selected_date
                                                            end_date=selected_date
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
}
