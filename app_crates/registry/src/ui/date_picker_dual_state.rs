use time::{Date, Month};

#[derive(PartialEq, Eq, Clone)]
pub struct DatePickerDualState {
    pub start_date: Date,
    pub end_date: Date,
}

impl DatePickerDualState {
    #[must_use]
    pub const fn new(start_date: Date, end_date: Date) -> Self {
        Self { start_date, end_date }
    }

    /// Handle day selection logic
    pub fn handle_day_selection(&mut self, day: u8, month: Month, year: i32) {
        if day == 0 {
            return;
        }

        let Some(new_date) = Date::from_calendar_date(year, month, day).ok() else {
            return;
        };

        // If clicking before or at start date, set as new start
        // Otherwise set as end date
        if new_date <= self.start_date {
            self.start_date = new_date;
        } else {
            self.end_date = new_date;
        }
    }

    /// Get the month and year for display (0 = first month, 1 = second month)
    #[must_use]
    pub fn get_display_month(display_date: Date, month_offset: i32) -> (Month, i32) {
        match month_offset {
            0 => (display_date.month(), display_date.year()),
            _ => next_month_year(display_date.month(), display_date.year()),
        }
    }

    /// Check if a date is start or end date
    #[must_use]
    pub fn is_start_or_end_date(&self, date: Date) -> bool {
        date == self.start_date || date == self.end_date
    }

    /// Calculates calendar data for the date picker
    #[must_use]
    pub fn calculate_calendar_data(year: i32, month: Month) -> Vec<(u8, Month, i32, bool, bool)> {
        let Some(first_day) = Date::from_calendar_date(year, month, 1).ok() else {
            return vec![];
        };
        let first_weekday = first_day.weekday().number_from_sunday() as usize - 1;

        let (prev_month_val, prev_year_val) = prev_month_year(month, year);
        let (next_month_val, next_year_val) = next_month_year(month, year);

        let days_in_prev_month = prev_month_val.length(prev_year_val);
        let days_in_month = month.length(year);

        let mut days = vec![];

        // Leading days from previous month.
        // `first_weekday` and `i` are both bounded to 0..7 (weekday index), so the
        // `u8` casts below never truncate.
        #[allow(clippy::cast_possible_truncation)]
        for i in 0..first_weekday {
            let day = days_in_prev_month - (first_weekday as u8) + (i as u8) + 1;
            days.push((day, prev_month_val, prev_year_val, false, true));
        }

        // Days in current month
        for day in 1..=days_in_month {
            days.push((day, month, year, false, false));
        }

        // Trailing days from next month to fill the last week.
        // `trailing` is `% 7`, so it is bounded to 0..7 and the `u8` cast never truncates.
        let trailing = (7 - days.len() % 7) % 7;
        #[allow(clippy::cast_possible_truncation)]
        for day in 1..=trailing as u8 {
            days.push((day, next_month_val, next_year_val, false, true));
        }

        days
    }
}

fn prev_month_year(month: Month, year: i32) -> (Month, i32) {
    if month == Month::January {
        (Month::December, year - 1)
    } else {
        (month.previous(), year)
    }
}

fn next_month_year(month: Month, year: i32) -> (Month, i32) {
    if month == Month::December {
        (Month::January, year + 1)
    } else {
        (month.next(), year)
    }
}
