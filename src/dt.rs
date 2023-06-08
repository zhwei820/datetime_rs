use chrono::{DateTime, Datelike, Duration, Local, Timelike, Weekday};

/// The common set of methods for date component.
pub trait WeekdayMisc {
    type T;
    /// Returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).
    fn succ_day(&self, target: &Self::T) -> i64;
}

impl WeekdayMisc for Weekday {
    type T = Weekday;
    fn succ_day(&self, target: &Weekday) -> i64 {
        let mut weekday = self;
        let mut tmp;
        for i in 0..7 {
            if weekday == target {
                return i;
            }
            tmp = weekday.succ();
            weekday = &tmp;
        }
        return -1;
    }
}

pub fn next_weekday(week: i64, weekday: &Weekday) -> DateTime<Local> {
    let now = Local::now();

    let days = now.weekday().succ_day(weekday);
    let duration = Duration::weeks(week) + Duration::days(days);

    let next_friday = now + duration;

    next_friday
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

// ====================

use once_cell::sync::Lazy;

static LayOut: Lazy<String> = Lazy::new(|| "%Y-%m-%d %H:%M:%S".to_string());

pub trait DateTimeFormat {
    /// Returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).
    fn format_default(&self) -> String;
}

impl DateTimeFormat for DateTime<Local> {
    fn format_default(&self) -> String {
        self.format(&LayOut).to_string()
    }
}
