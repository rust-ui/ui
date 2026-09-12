use time::{Date, Month};

#[derive(Debug, Clone, Copy)]
pub struct DatePickerDay {
    pub day: u8,
    pub disabled: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DatePickerState {
    pub start_date: Date,
    pub end_date: Date,
}

impl DatePickerState {
    pub fn new(start_date: Date, end_date: Date) -> Self {
        Self { start_date, end_date }
    }

    pub fn get_calendar_days(year: i32, month: Month) -> Vec<DatePickerDay> {
        let Some(first_day) = Date::from_calendar_date(year, month, 1).ok() else {
            return vec![];
        };
        let first_weekday = first_day.weekday().number_from_monday() as usize - 1;

        let (prev_month, prev_year) = prev_month_year(month, year);
        let days_in_prev_month = prev_month.length(prev_year);
        let days_in_month = month.length(year);

        let mut days = vec![];

        // `first_weekday` and `i` are both bounded to 0..7 (weekday index), so the
        // `u8` casts below never truncate.
        #[allow(clippy::cast_possible_truncation)]
        for i in 0..first_weekday {
            let day = days_in_prev_month - (first_weekday as u8) + (i as u8) + 1;
            days.push(DatePickerDay { day, disabled: true });
        }

        for day in 1..=days_in_month {
            days.push(DatePickerDay { day, disabled: false });
        }

        // `trailing` is `% 7`, so it is bounded to 0..7 and the `u8` cast never truncates.
        let trailing = (7 - days.len() % 7) % 7;
        #[allow(clippy::cast_possible_truncation)]
        for day in 1..=trailing as u8 {
            days.push(DatePickerDay { day, disabled: true });
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
