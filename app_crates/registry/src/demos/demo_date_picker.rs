use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use time::{Date, Month};

use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

#[component]
pub fn DemoDatePicker() -> Element {
    let default_start = Date::from_calendar_date(2025, Month::May, 5).unwrap();
    let default_end = Date::from_calendar_date(2025, Month::May, 14).unwrap();

    let mut start_date = use_signal(|| default_start);
    let mut end_date = use_signal(|| default_end);
    let mut display_date = use_signal(|| default_start);

    let go_to_previous_month = move |_| {
        let current = *display_date.read();
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
        let current = *display_date.read();
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
        let year = display_date.read().year();
        let month = display_date.read().month();
        let Ok(new_date) = Date::from_calendar_date(year, month, day) else { return };

        let current_start = *start_date.read();
        let current_end = *end_date.read();
        let days_from_start = (new_date - current_start).whole_days().abs();
        let days_from_end = (new_date - current_end).whole_days().abs();

        if days_from_start <= days_from_end {
            start_date.set(new_date);
        } else {
            end_date.set(new_date);
        }

        if *start_date.read() > *end_date.read() {
            let temp = *start_date.read();
            start_date.set(*end_date.read());
            end_date.set(temp);
        }
    };

    let year = display_date.read().year();
    let month = display_date.read().month();
    let days = DatePickerState::get_calendar_days(year, month);
    let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();
    let month_name = month.to_string();

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
                DatePickerTitle { role: "presentation", "{month_name} {year}" }
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
                    for week in weeks {
                        DatePickerRow {
                            for day_item in week {
                                DatePickerCell {
                                    day: day_item.day,
                                    year,
                                    month,
                                    disabled: day_item.disabled,
                                    start_date,
                                    end_date,
                                    on_click: handle_day_click,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
