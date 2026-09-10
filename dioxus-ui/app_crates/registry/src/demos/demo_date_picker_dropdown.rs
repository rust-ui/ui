use dioxus::prelude::*;
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
pub fn DemoDatePickerDropdown() -> Element {
    let today = OffsetDateTime::now_utc().date();
    let mut selected_date = use_signal(|| today);
    let mut display_date = use_signal(|| today);

    let years: Vec<i32> = (1924..=2030).collect();

    let handle_month_change = move |ev: Event<FormData>| {
        let val = ev.value().parse::<u8>().unwrap_or(1);
        let month = Month::try_from(val).unwrap_or(Month::January);
        let current = display_date();
        if let Ok(new_date) = Date::from_calendar_date(current.year(), month, 1) {
            display_date.set(new_date);
        }
    };

    let handle_year_change = move |ev: Event<FormData>| {
        let val = ev.value().parse::<i32>().unwrap_or(today.year());
        let current = display_date();
        if let Ok(new_date) = Date::from_calendar_date(val, current.month(), 1) {
            display_date.set(new_date);
        }
    };

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }
        let year = display_date().year();
        let month = display_date().month();
        if let Ok(new_date) = Date::from_calendar_date(year, month, day) {
            selected_date.set(new_date);
        }
    };

    let select_class = "h-7 rounded-md border border-input bg-transparent px-2 text-sm focus:outline-none focus:ring-1 focus:ring-ring";

    let year = display_date().year();
    let month = display_date().month();
    let days = DatePickerState::get_calendar_days(year, month);
    let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

    rsx! {
        DatePicker {
            DatePickerHeader { class: "flex gap-2",
                select { class: "flex-1 {select_class}", onchange: handle_month_change,
                    {MONTHS.iter().map(|(num, name)| {
                        let num = *num;
                        let selected = display_date().month() as u8 == num;
                        rsx! {
                            option { value: "{num}", selected: selected, {*name} }
                        }
                    })}
                }
                select { class: "w-[5.5rem] {select_class}", onchange: handle_year_change,
                    {years.iter().map(|&yr| {
                        let selected = display_date().year() == yr;
                        rsx! {
                            option { value: "{yr}", selected: selected, {yr.to_string()} }
                        }
                    })}
                }
            }

            table { class: "space-y-1 w-full border-collapse", role: "grid",
                thead {
                    tr { class: "flex",
                        DatePickerWeekDay { aria_label: "Monday", "Mo" }
                        DatePickerWeekDay { aria_label: "Tuesday", "Tu" }
                        DatePickerWeekDay { aria_label: "Wednesday", "We" }
                        DatePickerWeekDay { aria_label: "Thursday", "Th" }
                        DatePickerWeekDay { aria_label: "Friday", "Fr" }
                        DatePickerWeekDay { aria_label: "Saturday", "Sa" }
                        DatePickerWeekDay { aria_label: "Sunday", "Su" }
                    }
                }
                tbody {
                    {weeks.into_iter().map(|week| {
                        rsx! {
                            DatePickerRow {
                                {week.into_iter().map(|DatePickerDay { day, disabled }| {
                                    rsx! {
                                        DatePickerCell {
                                            day: day,
                                            year: year,
                                            month: month,
                                            disabled: disabled,
                                            start_date: selected_date,
                                            end_date: selected_date,
                                            on_click: handle_day_click,
                                        }
                                    }
                                })}
                            }
                        }
                    })}
                }
            }
        }
    }
}
