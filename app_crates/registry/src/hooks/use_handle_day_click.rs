use dioxus::prelude::*;
use time::Date;

/// Hook for managing date range selection
///
/// Returns a tuple of (start_date, end_date, handle_day_click) where:
/// - `start_date`: Signal<Date> for the start date
/// - `end_date`: Signal<Date> for the end date
/// - `handle_day_click`: Function that takes a day number and updates the closest date
pub fn use_handle_day_click(
    initial_start: Date,
    initial_end: Date,
) -> (Signal<Date>, Signal<Date>, impl FnMut(u8) + Clone) {
    let mut start_date_signal = use_signal(|| initial_start);
    let mut end_date_signal = use_signal(|| initial_end);

    let handle_day_click = move |day: u8| {
        if day == 0 {
            return;
        }

        let year = start_date_signal().year();
        let month = start_date_signal().month();

        // Create new date for the selected day
        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else { return };

        // Determine which date to update based on proximity
        let current_start = start_date_signal().day();
        let current_end = end_date_signal().day();

        if current_start.abs_diff(day) <= current_end.abs_diff(day) {
            start_date_signal.set(new_date);
        } else {
            end_date_signal.set(new_date);
        }
    };

    (start_date_signal, end_date_signal, handle_day_click)
}
