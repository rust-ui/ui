use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use time::{Date, Month};

use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay, DatePickerWeekNumberCell, DatePickerWeekNumberHeader,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

fn prev_month_year(month: Month, year: i32) -> (Month, i32) {
    if month == Month::January { (Month::December, year - 1) } else { (month.previous(), year) }
}

fn next_month_year(month: Month, year: i32) -> (Month, i32) {
    if month == Month::December { (Month::January, year + 1) } else { (month.next(), year) }
}

#[component]
pub fn DemoDatePickerWeekNumbers() -> Element {
    let Some(default_start) = Date::from_calendar_date(2025, Month::May, 5).ok() else {
        return rsx! { p { "Invalid default start date" } };
    };
    let Some(default_end) = Date::from_calendar_date(2025, Month::May, 14).ok() else {
        return rsx! { p { "Invalid default end date" } };
    };

    let mut start_date_signal = use_signal(|| default_start);
    let mut end_date_signal = use_signal(|| default_end);
    let mut display_date_signal = use_signal(|| default_start);

    let go_to_previous_month = move |_| {
        let current = display_date_signal();
        let (prev_month, prev_year) = prev_month_year(current.month(), current.year());
        if let Ok(new_date) = Date::from_calendar_date(prev_year, prev_month, 1) {
            display_date_signal.set(new_date);
        }
    };

    let go_to_next_month = move |_| {
        let current = display_date_signal();
        let (next_month, next_year) = next_month_year(current.month(), current.year());
        if let Ok(new_date) = Date::from_calendar_date(next_year, next_month, 1) {
            display_date_signal.set(new_date);
        }
    };

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }
        let year = display_date_signal().year();
        let month = display_date_signal().month();
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        let current_start = start_date_signal();
        let current_end = end_date_signal();
        let days_from_start = (new_date - current_start).whole_days().abs();
        let days_from_end = (new_date - current_end).whole_days().abs();

        if days_from_start <= days_from_end {
            start_date_signal.set(new_date);
        } else {
            end_date_signal.set(new_date);
        }

        if start_date_signal() > end_date_signal() {
            let temp = start_date_signal();
            start_date_signal.set(end_date_signal());
            end_date_signal.set(temp);
        }
    };

    let year = display_date_signal().year();
    let month = display_date_signal().month();
    let days = DatePickerState::get_calendar_days(year, month);
    let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

    rsx! {
        DatePicker {
            DatePickerHeader {
                DatePickerNavButton {
                    class: "justify-self-start",
                    title: "previous-month",
                    aria_label: "Go to previous month",
                    onclick: go_to_previous_month,
                    ChevronLeft {}
                }
                DatePickerTitle { role: "presentation",
                    {display_date_signal().month().to_string()}
                    " "
                    {display_date_signal().year().to_string()}
                }
                DatePickerNavButton {
                    class: "justify-self-end",
                    title: "next-month",
                    aria_label: "Go to next month",
                    onclick: go_to_next_month,
                    ChevronRight {}
                }
            }

            table { class: "space-y-1 w-full border-collapse", role: "grid",
                thead {
                    tr { class: "flex",
                        DatePickerWeekNumberHeader { aria_label: "Week", "#" }
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
                        let week_num = week
                            .iter()
                            .find(|d| !d.disabled)
                            .and_then(|d| Date::from_calendar_date(year, month, d.day).ok())
                            .map(|d| d.iso_week());
                        rsx! {
                            DatePickerRow {
                                DatePickerWeekNumberCell {
                                    {week_num.map(|n| n.to_string()).unwrap_or_default()}
                                }
                                {week.into_iter().map(|DatePickerDay { day, disabled }| {
                                    rsx! {
                                        DatePickerCell {
                                            day: day,
                                            year: year,
                                            month: month,
                                            disabled: disabled,
                                            start_date: start_date_signal,
                                            end_date: end_date_signal,
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
