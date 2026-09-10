use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use time::{Date, Month};

use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

#[component]
pub fn DemoDatePickerDual() -> Element {
    // Initialize dates from defaults
    let Some(default_start) = Date::from_calendar_date(2025, Month::May, 5).ok() else {
        return rsx! { p { "Invalid default start date" } };
    };
    let Some(default_end) = Date::from_calendar_date(2025, Month::June, 8).ok() else {
        return rsx! { p { "Invalid default end date" } };
    };

    let start_date_signal = use_signal(|| default_start);
    let end_date_signal = use_signal(|| default_end);

    // Track the left calendar month (right will be left + 1)
    let mut left_display_date_signal = use_signal(|| default_start);

    // Navigation: Go to previous 2 months
    let go_to_previous = move |_| {
        let current = left_display_date_signal();
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
        let current = left_display_date_signal();
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
        let mut start_signal = start_date_signal;
        let mut end_signal = end_date_signal;

        // If clicking before or at start date, set as new start
        // Otherwise set as end date
        if new_date <= start_signal() {
            start_signal.set(new_date);
        } else {
            end_signal.set(new_date);
        }
    };

    // Helper to get display date for left/right calendar
    let left = left_display_date_signal();
    let left_display = left;
    let right_display = (if left.month() == Month::December {
        Date::from_calendar_date(left.year() + 1, Month::January, 1)
    } else {
        Date::from_calendar_date(left.year(), left.month().next(), 1)
    })
    .unwrap_or(left);

    let left_year = left_display.year();
    let left_month = left_display.month();
    let right_year = right_display.year();
    let right_month = right_display.month();

    let left_days = DatePickerState::get_calendar_days(left_year, left_month);
    let left_weeks: Vec<Vec<DatePickerDay>> = left_days.chunks(7).map(|chunk| chunk.to_vec()).collect();
    let right_days = DatePickerState::get_calendar_days(right_year, right_month);
    let right_weeks: Vec<Vec<DatePickerDay>> = right_days.chunks(7).map(|chunk| chunk.to_vec()).collect();

    rsx! {
        div { class: "flex gap-4",
            // Left calendar
            DatePicker {
                DatePickerHeader {
                    DatePickerNavButton {
                        class: "justify-self-start",
                        title: "previous-months",
                        aria_label: "Go to previous months",
                        onclick: go_to_previous,
                        ChevronLeft {}
                    }
                    DatePickerTitle { role: "presentation",
                        {left_month.to_string()}
                        " "
                        {left_year.to_string()}
                    }
                    span {}
                }

                table { class: "space-y-1 w-full border-collapse", role: "grid",
                    thead {
                        tr { class: "flex",
                            DatePickerWeekDay { aria_label: "Sunday", "Su" }
                            DatePickerWeekDay { aria_label: "Monday", "Mo" }
                            DatePickerWeekDay { aria_label: "Tuesday", "Tu" }
                            DatePickerWeekDay { aria_label: "Wednesday", "We" }
                            DatePickerWeekDay { aria_label: "Thursday", "Th" }
                            DatePickerWeekDay { aria_label: "Friday", "Fr" }
                            DatePickerWeekDay { aria_label: "Saturday", "Sa" }
                        }
                    }
                    tbody {
                        {left_weeks.into_iter().map(|week| {
                            rsx! {
                                DatePickerRow {
                                    {week.into_iter().map(|DatePickerDay { day, disabled }| {
                                        rsx! {
                                            DatePickerCell {
                                                day: day,
                                                year: left_year,
                                                month: left_month,
                                                disabled: disabled,
                                                start_date: start_date_signal,
                                                end_date: end_date_signal,
                                                on_click: move |d| handle_day_click(left_year, left_month, d),
                                            }
                                        }
                                    })}
                                }
                            }
                        })}
                    }
                }
            }

            // Right calendar
            DatePicker {
                DatePickerHeader {
                    span {}
                    DatePickerTitle { role: "presentation",
                        {right_month.to_string()}
                        " "
                        {right_year.to_string()}
                    }
                    DatePickerNavButton {
                        class: "justify-self-end",
                        title: "next-months",
                        aria_label: "Go to next months",
                        onclick: go_to_next,
                        ChevronRight {}
                    }
                }

                table { class: "space-y-1 w-full border-collapse", role: "grid",
                    thead {
                        tr { class: "flex",
                            DatePickerWeekDay { aria_label: "Sunday", "Su" }
                            DatePickerWeekDay { aria_label: "Monday", "Mo" }
                            DatePickerWeekDay { aria_label: "Tuesday", "Tu" }
                            DatePickerWeekDay { aria_label: "Wednesday", "We" }
                            DatePickerWeekDay { aria_label: "Thursday", "Th" }
                            DatePickerWeekDay { aria_label: "Friday", "Fr" }
                            DatePickerWeekDay { aria_label: "Saturday", "Sa" }
                        }
                    }
                    tbody {
                        {right_weeks.into_iter().map(|week| {
                            rsx! {
                                DatePickerRow {
                                    {week.into_iter().map(|DatePickerDay { day, disabled }| {
                                        rsx! {
                                            DatePickerCell {
                                                day: day,
                                                year: right_year,
                                                month: right_month,
                                                disabled: disabled,
                                                start_date: start_date_signal,
                                                end_date: end_date_signal,
                                                on_click: move |d| handle_day_click(right_year, right_month, d),
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
}
