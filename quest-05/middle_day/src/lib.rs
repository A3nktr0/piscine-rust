use chrono::{Datelike, NaiveDate};
pub use chrono::{Utc, TimeZone, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    match NaiveDate::from_ymd_opt(year, 2, 29).is_some() {
        true => None,
        false => NaiveDate::from_ymd_opt(year, 7, 2).map(|date| date.weekday()),
    }
}
