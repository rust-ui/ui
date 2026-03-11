use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;
use time::{Date, Duration, Month, OffsetDateTime};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardFooter};
use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow, DatePickerTitle,
    DatePickerWeekDay,
};
use crate::ui::date_picker_state::{DatePickerDay, DatePickerState};

#[component]
pub fn DemoDatePickerPresets() -> impl IntoView {
    let today = OffsetDateTime::now_utc().date();

    let selected_date = RwSignal::new(today);
    let display_date = RwSignal::new(today);

    let go_to_previous_month = move |_| {
        let current = display_date.get();
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
        let current = display_date.get();
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
        let year = display_date.get().year();
        let month = display_date.get().month();
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };
        selected_date.set(new_date);
    };

    let apply_preset = move |days: i64| {
        let preset_date = today + Duration::days(days);
        selected_date.set(preset_date);
        let Some(month_start) = Date::from_calendar_date(preset_date.year(), preset_date.month(), 1).ok() else {
            return;
        };
        display_date.set(month_start);
    };

    let presets: Vec<(&str, i64)> =
        vec![("Today", 0), ("Tomorrow", 1), ("In 3 days", 3), ("In a week", 7), ("In 2 weeks", 14)];

    view! {
        <Card class="w-fit max-w-[300px]">
            <CardContent>
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
                            {move || display_date.get().month().to_string()} {move || display_date.get().year()}
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
                                let year = display_date.get().year();
                                let month = display_date.get().month();
                                let days = DatePickerState::get_calendar_days(year, month);
                                let weeks: Vec<Vec<DatePickerDay>> = days
                                    .chunks(7)
                                    .map(|chunk| chunk.to_vec())
                                    .collect();

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
                                                        key=|DatePickerDay { day, disabled }| {
                                                            format!("{day}-{disabled}")
                                                        }
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
            </CardContent>
            <CardFooter class="flex flex-wrap gap-2 border-t">
                {presets
                    .into_iter()
                    .map(|(label, days)| {
                        view! {
                            <Button
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Sm
                                class="flex-1"
                                on:click=move |_| apply_preset(days)
                            >
                                {label}
                            </Button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </CardFooter>
        </Card>
    }
}
