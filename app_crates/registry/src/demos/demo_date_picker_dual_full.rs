use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;
use time::{Date, Month};

use crate::ui::date_picker::{
    DatePicker, DatePickerCell, DatePickerHeader, DatePickerMonth, DatePickerNavButton, DatePickerRow, DatePickerTable,
    DatePickerTitle, DatePickerWeekDay,
};
use crate::ui::date_picker_dual_state::DatePickerDualState;
use crate::utils::query::QueryUtils;

#[component]
pub fn DemoDatePickerDualFull() -> impl IntoView {
    let (state, setup_url_sync) = DatePickerDualState::from_url_or_default();

    setup_url_sync(); // Set up URL synchronization

    // Extract start and end dates as RwSignal
    let start_date_signal = RwSignal::new(state.get_untracked().start_date);
    let end_date_signal = RwSignal::new(state.get_untracked().end_date);

    // Track which month is currently displayed (independent from selected dates)
    let initial_display =
        Date::from_calendar_date(state.get_untracked().start_date.year(), state.get_untracked().start_date.month(), 1)
            .unwrap_or(state.get_untracked().start_date);
    let display_month_date_signal = RwSignal::new(initial_display);

    // Sync local signals with state when URL changes
    Effect::new(move |_| {
        let s = state.get();
        start_date_signal.set(s.start_date);
        end_date_signal.set(s.end_date);
        // Don't reset display_month_date_signal here - let user navigation control it
    });

    // Navigation: Go to previous month (jump 2 months)
    let go_to_previous_month = move |_| {
        let current = display_month_date_signal.get();
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
        let current = display_month_date_signal.get();
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
        state.update(|state| state.handle_day_selection(day, month, year));

        let (start, end) = state.with_untracked(|s| (s.start_date, s.end_date));
        QueryUtils::update_dates_url(Some(start), Some(end));
    };

    view! {
        <DatePicker class="w-fit">
            <DatePickerHeader>
                <DatePickerNavButton
                    class="justify-self-start"
                    attr:title="previous-month"
                    attr:aria-label="Go to previous month"
                    on:click=go_to_previous_month
                >
                    <ChevronLeft />
                </DatePickerNavButton>

                <div class="flex flex-1 gap-8 justify-around">
                    <DatePickerTitle attr:role="presentation">
                        {move || {
                            let (month, year) = DatePickerDualState::get_display_month(
                                display_month_date_signal.get(),
                                0,
                            );
                            format!("{} {}", month, year)
                        }}
                    </DatePickerTitle>
                    <DatePickerTitle attr:role="presentation">
                        {move || {
                            let (month, year) = DatePickerDualState::get_display_month(
                                display_month_date_signal.get(),
                                1,
                            );
                            format!("{} {}", month, year)
                        }}
                    </DatePickerTitle>
                </div>

                <DatePickerNavButton
                    class="justify-self-end"
                    attr:title="next-month"
                    attr:aria-label="Go to next month"
                    on:click=go_to_next_month
                >
                    <ChevronRight />
                </DatePickerNavButton>
            </DatePickerHeader>

            <div class="flex gap-8 mt-2">
                <DatePickerMonth>
                    <DatePickerTable attr:role="grid">
                        <thead>
                            <tr class="flex">
                                <DatePickerWeekDay attr:aria-label="Sunday">Su</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Monday">Mo</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Tuesday">Tu</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Wednesday">We</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Thursday">Th</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Friday">Fr</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Saturday">Sa</DatePickerWeekDay>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let (target_month, target_year) = DatePickerDualState::get_display_month(
                                    display_month_date_signal.get(),
                                    0,
                                );
                                let days = DatePickerDualState::calculate_calendar_data(target_year, target_month);
                                days.chunks(7)
                                    .map(|week| {
                                        let week_cells = week
                                            .iter()
                                            .map(|&(day, cell_month, cell_year, disabled, _outside)| {
                                                view! {
                                                    <DatePickerCell
                                                        day=day
                                                        year=cell_year
                                                        month=cell_month
                                                        disabled=disabled
                                                        start_date=start_date_signal
                                                        end_date=end_date_signal
                                                        on_click=move |d| {
                                                            if !disabled {
                                                                handle_day_click(d, cell_month, cell_year);
                                                            }
                                                        }
                                                    />
                                                }
                                            })
                                            .collect_view();
                                        view! { <DatePickerRow>{week_cells}</DatePickerRow> }
                                    })
                                    .collect_view()
                            }}
                        </tbody>
                    </DatePickerTable>
                </DatePickerMonth>
                <DatePickerMonth>
                    <DatePickerTable attr:role="grid">
                        <thead>
                            <tr class="flex">
                                <DatePickerWeekDay attr:aria-label="Sunday">Su</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Monday">Mo</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Tuesday">Tu</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Wednesday">We</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Thursday">Th</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Friday">Fr</DatePickerWeekDay>
                                <DatePickerWeekDay attr:aria-label="Saturday">Sa</DatePickerWeekDay>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let (target_month, target_year) = DatePickerDualState::get_display_month(
                                    display_month_date_signal.get(),
                                    1,
                                );
                                let days = DatePickerDualState::calculate_calendar_data(target_year, target_month);
                                days.chunks(7)
                                    .map(|week| {
                                        let week_cells = week
                                            .iter()
                                            .map(|&(day, cell_month, cell_year, disabled, _outside)| {
                                                view! {
                                                    <DatePickerCell
                                                        day=day
                                                        year=cell_year
                                                        month=cell_month
                                                        disabled=disabled
                                                        start_date=start_date_signal
                                                        end_date=end_date_signal
                                                        on_click=move |d| {
                                                            if !disabled {
                                                                handle_day_click(d, cell_month, cell_year);
                                                            }
                                                        }
                                                    />
                                                }
                                            })
                                            .collect_view();
                                        view! { <DatePickerRow>{week_cells}</DatePickerRow> }
                                    })
                                    .collect_view()
                            }}
                        </tbody>
                    </DatePickerTable>
                </DatePickerMonth>
            </div>
        </DatePicker>
    }
}
