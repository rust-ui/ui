use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight, Clock2};
use time::{Date, Month, OffsetDateTime};

use crate::ui::card::{Card, CardContent, CardFooter};
use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};
use crate::ui::field::{Field, FieldGroup, FieldLabel, FieldVariant};
use crate::ui::input::InputType;
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoDatePickerTime() -> Element {
    let today = OffsetDateTime::now_utc().date();
    let mut selected_date = use_signal(|| today);
    let mut display_date = use_signal(|| today);

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
            CardContent {
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
            CardFooter { class: "pt-4 border-t",
                FieldGroup { class: "flex-row gap-4",
                    Field { variant: FieldVariant::Vertical,
                        FieldLabel { "Start Time" }
                        InputGroup {
                            InputGroupInput {
                                r#type: InputType::Time,
                                step: "1",
                                value: "10:30:00",
                                class: "[&::-webkit-calendar-picker-indicator]:hidden",
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                Clock2 {}
                            }
                        }
                    }
                    Field { variant: FieldVariant::Vertical,
                        FieldLabel { "End Time" }
                        InputGroup {
                            InputGroupInput {
                                r#type: InputType::Time,
                                step: "1",
                                value: "11:30:00",
                                class: "[&::-webkit-calendar-picker-indicator]:hidden",
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                Clock2 {}
                            }
                        }
                    }
                }
            }
        }
    }
}
