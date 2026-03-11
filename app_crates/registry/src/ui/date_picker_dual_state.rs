use leptos::prelude::*;
use time::{Date, Month};

use crate::utils::date::DateUtils;
use crate::utils::query::{QUERY, QueryUtils};

#[derive(PartialEq, Clone)]
pub struct DatePickerDualState {
    pub start_date: Date,
    pub end_date: Date,
}

impl DatePickerDualState {
    pub fn new(start_date: Date, end_date: Date) -> Self {
        Self { start_date, end_date }
    }

    /// Create a new state from URL parameters with fallback defaults
    pub fn from_url_or_default() -> (RwSignal<Self>, impl Fn() + Clone) {
        let start_date_query = QueryUtils::extract(QUERY::START_DATE.to_string());
        let end_date_query = QueryUtils::extract(QUERY::END_DATE.to_string());

        let initial_state = Memo::new(move |_| {
            let start = start_date_query.get();
            let end = end_date_query.get();

            let fallback_start = Date::from_calendar_date(2025, Month::May, 5).unwrap_or(Date::MIN);
            let fallback_end = Date::from_calendar_date(2025, Month::May, 14).unwrap_or(Date::MIN);
            let start_date = DateUtils::parse_from_url(start).unwrap_or(fallback_start);
            let end_date = DateUtils::parse_from_url(end).unwrap_or(fallback_end);

            Self::new(start_date, end_date)
        });

        // Use RwSignal with the derived initial state
        let state_signal = RwSignal::new(initial_state.get());

        // Create a cleanup function that sets up the effect
        let setup_url_sync = {
            move || {
                // Update state when URL parameters change
                Effect::new(move |_| {
                    let new_state = initial_state.get();
                    state_signal.set(new_state);
                });
            }
        };

        (state_signal, setup_url_sync)
    }

    /// Handle day selection logic
    pub fn handle_day_selection(&mut self, day: u8, month: Month, year: i32) {
        if day == 0 {
            return;
        }

        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        // If clicking before or at start date, set as new start
        // Otherwise set as end date
        if new_date <= self.start_date {
            self.start_date = new_date;
        } else {
            self.end_date = new_date;
        }
    }

    /// Get the month and year for display (0 = first month, 1 = second month)
    pub fn get_display_month(display_date: Date, month_offset: i32) -> (Month, i32) {
        match month_offset {
            0 => (display_date.month(), display_date.year()),
            _ => DateUtils::next_month_year(display_date.month(), display_date.year()),
        }
    }

    /// Check if a date is start or end date
    pub fn is_start_or_end_date(&self, date: Date) -> bool {
        date == self.start_date || date == self.end_date
    }

    /// Calculates calendar data for the date picker
    pub fn calculate_calendar_data(year: i32, month: Month) -> Vec<(u8, Month, i32, bool, bool)> {
        let Some(first_day) = Date::from_calendar_date(year, month, 1).ok() else { return vec![] };
        let first_weekday = first_day.weekday().number_from_sunday() as usize - 1;

        let (prev_month_val, prev_year_val) = DateUtils::prev_month_year(month, year);
        let (next_month_val, next_year_val) = DateUtils::next_month_year(month, year);

        let days_in_prev_month = prev_month_val.length(prev_year_val);
        let days_in_month = month.length(year);

        let mut days = vec![];

        // Leading days from previous month
        for i in 0..first_weekday {
            let day = days_in_prev_month - (first_weekday as u8) + (i as u8) + 1;
            days.push((day, prev_month_val, prev_year_val, false, true));
        }

        // Days in current month
        for day in 1..=days_in_month {
            days.push((day, month, year, false, false));
        }

        // Trailing days from next month to fill the last week
        let trailing = (7 - days.len() % 7) % 7;
        for day in 1..=trailing as u8 {
            days.push((day, next_month_val, next_year_val, false, true));
        }

        days
    }
}
