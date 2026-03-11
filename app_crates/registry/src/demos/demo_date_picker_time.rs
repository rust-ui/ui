use icons::{ChevronLeft, ChevronRight, Clock2};
use leptos::prelude::*;
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
pub fn DemoDatePickerTime() -> impl IntoView {
    let today = OffsetDateTime::now_utc().date();
    let selected_date = RwSignal::new(today);
    let display_date = RwSignal::new(today);

    let go_to_previous_month = move |_| {
        let current = display_date.get();
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
        let current = display_date.get();
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
        let year = display_date.get().year();
        let month = display_date.get().month();
        if let Ok(new_date) = Date::from_calendar_date(year, month, day) {
            selected_date.set(new_date);
        }
    };

    view! {
        <Card class="w-fit">
            <CardContent>
                <DatePicker>
                    <DatePickerHeader>
                        <DatePickerNavButton
                            class="justify-self-start"
                            attr:aria-label="Go to previous month"
                            on:click=go_to_previous_month
                        >
                            <ChevronLeft />
                        </DatePickerNavButton>
                        <DatePickerTitle attr:role="presentation">
                            {move || display_date.get().month().to_string()} " " {move || display_date.get().year()}
                        </DatePickerTitle>
                        <DatePickerNavButton
                            class="justify-self-end"
                            attr:aria-label="Go to next month"
                            on:click=go_to_next_month
                        >
                            <ChevronRight />
                        </DatePickerNavButton>
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
            <CardFooter class="pt-4 border-t">
                <FieldGroup class="flex-row gap-4">
                    <Field variant=FieldVariant::Vertical>
                        <FieldLabel>"Start Time"</FieldLabel>
                        <InputGroup>
                            <InputGroupInput
                                r#type=InputType::Time
                                step="1"
                                attr:value="10:30:00"
                                class="[&::-webkit-calendar-picker-indicator]:hidden"
                            />
                            <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                                <Clock2 />
                            </InputGroupAddon>
                        </InputGroup>
                    </Field>
                    <Field variant=FieldVariant::Vertical>
                        <FieldLabel>"End Time"</FieldLabel>
                        <InputGroup>
                            <InputGroupInput
                                r#type=InputType::Time
                                step="1"
                                attr:value="11:30:00"
                                class="[&::-webkit-calendar-picker-indicator]:hidden"
                            />
                            <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                                <Clock2 />
                            </InputGroupAddon>
                        </InputGroup>
                    </Field>
                </FieldGroup>
            </CardFooter>
        </Card>
    }
}
