use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use time::{Date, Month, OffsetDateTime};

use crate::ui::card::{Card, CardContent};
use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

/// Days 12–26 of the booked month are marked as booked (struck through, not selectable).
fn is_booked_day(year: i32, month: Month, day: u8, booked_year: i32, booked_month: Month) -> bool {
    day > 0 && year == booked_year && month == booked_month && (12..=26).contains(&day)
}

#[component]
pub fn DemoDatePickerBooked() -> Element {
    let current_year = OffsetDateTime::now_utc().date().year();

    let booked_year = current_year;
    let booked_month = Month::February;

    let initial_selected =
        Date::from_calendar_date(current_year, Month::February, 3).unwrap_or_else(|_| OffsetDateTime::now_utc().date());
    let initial_display =
        Date::from_calendar_date(current_year, Month::February, 1).unwrap_or_else(|_| OffsetDateTime::now_utc().date());

    let mut selected_date = use_signal(|| initial_selected);
    let mut display_date = use_signal(|| initial_display);

    let go_to_previous_month = move |_| {
        let current = display_date();
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
        let current = display_date();
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
        let year = display_date().year();
        let month = display_date().month();
        if let Ok(new_date) = Date::from_calendar_date(year, month, day) {
            selected_date.set(new_date);
        }
    };

    let year = display_date().year();
    let month = display_date().month();
    let days = DatePickerState::get_calendar_days(year, month);
    let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

    rsx! {
        Card { class: "w-fit",
            CardContent { class: "p-0",
                DatePicker {
                    DatePickerHeader {
                        DatePickerNavButton {
                            class: "justify-self-start",
                            aria_label: "Go to previous month",
                            onclick: go_to_previous_month,
                            ChevronLeft {}
                        }
                        DatePickerTitle { role: "presentation",
                            {display_date().month().to_string()}
                            " "
                            {display_date().year().to_string()}
                        }
                        DatePickerNavButton {
                            class: "justify-self-end",
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
                            {weeks.into_iter().map(|week| {
                                rsx! {
                                    DatePickerRow {
                                        {week.into_iter().map(|DatePickerDay { day, disabled }| {
                                            let booked = !disabled
                                                && is_booked_day(year, month, day, booked_year, booked_month);
                                            let cell_class = if booked {
                                                "line-through aria-disabled:opacity-100"
                                            } else {
                                                ""
                                            };
                                            rsx! {
                                                DatePickerCell {
                                                    day: day,
                                                    year: year,
                                                    month: month,
                                                    disabled: disabled || booked,
                                                    start_date: selected_date,
                                                    end_date: selected_date,
                                                    on_click: handle_day_click,
                                                    class: cell_class,
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
}
