use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use time::{Date, Duration, Month, OffsetDateTime};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardFooter};
use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

#[component]
pub fn DemoDatePickerPresets() -> Element {
    let today = OffsetDateTime::now_utc().date();

    let selected_date = use_signal(|| today);
    let mut display_date = use_signal(|| today);

    let go_to_previous_month = move |_| {
        let current = display_date();
        let Some(new_date) = (if current.month() == Month::January {
            Date::from_calendar_date(current.year() - 1, Month::December, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().previous(), 1)
        })
        .ok() else {
            return;
        };
        display_date.set(new_date);
    };

    let go_to_next_month = move |_| {
        let current = display_date();
        let Some(new_date) = (if current.month() == Month::December {
            Date::from_calendar_date(current.year() + 1, Month::January, 1)
        } else {
            Date::from_calendar_date(current.year(), current.month().next(), 1)
        })
        .ok() else {
            return;
        };
        display_date.set(new_date);
    };

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }
        let year = display_date().year();
        let month = display_date().month();
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };
        let mut selected_signal = selected_date;
        selected_signal.set(new_date);
    };

    let apply_preset = move |days: i64| {
        let preset_date = today + Duration::days(days);
        let mut selected_signal = selected_date;
        selected_signal.set(preset_date);
        let Some(month_start) = Date::from_calendar_date(preset_date.year(), preset_date.month(), 1).ok() else {
            return;
        };
        let mut display_signal = display_date;
        display_signal.set(month_start);
    };

    let presets: Vec<(&str, i64)> =
        vec![("Today", 0), ("Tomorrow", 1), ("In 3 days", 3), ("In a week", 7), ("In 2 weeks", 14)];

    let year = display_date().year();
    let month = display_date().month();
    let days = DatePickerState::get_calendar_days(year, month);
    let weeks: Vec<Vec<DatePickerDay>> = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

    rsx! {
        Card { class: "w-fit max-w-[300px]",
            CardContent {
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
                            {display_date().month().to_string()}
                            " "
                            {display_date().year().to_string()}
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
                                    for DatePickerDay { day, disabled } in week {
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
                                }
                            }
                        }
                    }
                }
            }
            CardFooter { class: "flex flex-wrap gap-2 border-t",
                for (label, days) in presets {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "flex-1",
                        onclick: move |_| apply_preset(days),
                        {label}
                    }
                }
            }
        }
    }
}
