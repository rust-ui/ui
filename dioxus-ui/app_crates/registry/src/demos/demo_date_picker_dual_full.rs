use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use time::{Date, Month};

use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerMonth, DatePickerNavButton, DatePickerRow, DatePickerTable,
    DatePickerTitle, DatePickerWeekDay,
};
use crate::ui::date_picker_dual_state::DatePickerDualState;

#[component]
pub fn DemoDatePickerDualFull() -> Element {
    let fallback_start = Date::from_calendar_date(2025, Month::May, 5).unwrap_or(Date::MIN);
    let fallback_end = Date::from_calendar_date(2025, Month::May, 14).unwrap_or(Date::MIN);

    let state_signal = use_signal(|| DatePickerDualState::new(fallback_start, fallback_end));

    // Extract start and end dates as signals
    let mut start_date_signal = use_signal(|| fallback_start);
    let mut end_date_signal = use_signal(|| fallback_end);

    // Track which month is currently displayed
    let initial_display =
        Date::from_calendar_date(fallback_start.year(), fallback_start.month(), 1).unwrap_or(fallback_start);
    let mut display_month_date_signal = use_signal(|| initial_display);

    // Sync local signals with state
    use_effect(move || {
        let s = state_signal();
        start_date_signal.set(s.start_date);
        end_date_signal.set(s.end_date);
    });

    // Navigation: Go to previous month (jump 2 months)
    let go_to_previous_month = move |_| {
        let current = display_month_date_signal();
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
        display_month_date_signal.set(new_date);
    };

    // Navigation: Go to next month (jump 2 months)
    let go_to_next_month = move |_| {
        let current = display_month_date_signal();
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
        display_month_date_signal.set(new_date);
    };

    let handle_day_click = move |day: u8, month: Month, year: i32| {
        let mut state = state_signal;
        state.with_mut(|state| state.handle_day_selection(day, month, year));
        let s = state();
        let mut start_signal = start_date_signal;
        start_signal.set(s.start_date);
        let mut end_signal = end_date_signal;
        end_signal.set(s.end_date);
    };

    let (left_month, left_year) = DatePickerDualState::get_display_month(display_month_date_signal(), 0);
    let (right_month, right_year) = DatePickerDualState::get_display_month(display_month_date_signal(), 1);

    let left_days = DatePickerDualState::calculate_calendar_data(left_year, left_month);
    let right_days = DatePickerDualState::calculate_calendar_data(right_year, right_month);

    rsx! {
        DatePicker { class: "w-fit",
            DatePickerHeader {
                DatePickerNavButton {
                    class: "justify-self-start",
                    title: "previous-month",
                    aria_label: "Go to previous month",
                    onclick: go_to_previous_month,
                    ChevronLeft {}
                }

                div { class: "flex flex-1 gap-8 justify-around",
                    DatePickerTitle { role: "presentation",
                        {format!("{} {}", left_month, left_year)}
                    }
                    DatePickerTitle { role: "presentation",
                        {format!("{} {}", right_month, right_year)}
                    }
                }

                DatePickerNavButton {
                    class: "justify-self-end",
                    title: "next-month",
                    aria_label: "Go to next month",
                    onclick: go_to_next_month,
                    ChevronRight {}
                }
            }

            div { class: "flex gap-8 mt-2",
                DatePickerMonth {
                    DatePickerTable { role: "grid",
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
                            {left_days.chunks(7).map(|week| {
                                let week_vec: Vec<_> = week.to_vec();
                                rsx! {
                                    DatePickerRow {
                                        {week_vec.into_iter().map(|(day, cell_month, cell_year, disabled, _outside)| {
                                            rsx! {
                                                DatePickerCell {
                                                    day: day,
                                                    year: cell_year,
                                                    month: cell_month,
                                                    disabled: disabled,
                                                    start_date: start_date_signal,
                                                    end_date: end_date_signal,
                                                    on_click: move |d| {
                                                        if !disabled {
                                                            handle_day_click(d, cell_month, cell_year);
                                                        }
                                                    },
                                                }
                                            }
                                        })}
                                    }
                                }
                            })}
                        }
                    }
                }
                DatePickerMonth {
                    DatePickerTable { role: "grid",
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
                            {right_days.chunks(7).map(|week| {
                                let week_vec: Vec<_> = week.to_vec();
                                rsx! {
                                    DatePickerRow {
                                        {week_vec.into_iter().map(|(day, cell_month, cell_year, disabled, _outside)| {
                                            rsx! {
                                                DatePickerCell {
                                                    day: day,
                                                    year: cell_year,
                                                    month: cell_month,
                                                    disabled: disabled,
                                                    start_date: start_date_signal,
                                                    end_date: end_date_signal,
                                                    on_click: move |d| {
                                                        if !disabled {
                                                            handle_day_click(d, cell_month, cell_year);
                                                        }
                                                    },
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
