use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub type DateTimeUnit = i64;

/// A date and time struct that can be used to represent a time in milliseconds.
/// Expected usage is to use one of the following methods to create a new DateTime struct:
/// - `DateTime::now()`
/// - `DateTime::new(year, month, day, hour, minutes, seconds)`
/// - `DateTime::from_unix_epoch_milliseconds(milliseconds)`
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let datetime = DateTime::now();
/// let datetime_display = datetime.format();
/// println!("{}", datetime_display);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateTime {
    /// Milliseconds since the start of AD 0.
    milliseconds: DateTimeUnit,
}

#[cfg(feature = "sqlx")]
impl sqlx::FromRow for DateTime {}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl DateTime {
    /// Create a new `DateTime` from the current system time.
    /// Time is UTC.
    pub fn now() -> Self {
        Self::from_milliseconds(now_milliseconds())
    }

    /// Create a new `DateTime` from the provided values.
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minutes: u8, seconds: u8) -> Self {
        let is_leap_year = is_leap_year(year);
        let year = year as DateTimeUnit;
        let year_offset = year;
        let mut days = year_offset * 365;
        if year_offset > 0 {
            days += (year_offset - 1) / 4;
        }
        if year_offset > 0 {
            days -= (year_offset - 1) / 100;
        }
        if year_offset > 0 {
            days += (year_offset - 1) / 400;
        }
        let days_in_month = [
            31,
            if is_leap_year { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        for i in 0..(month - 1) {
            days += days_in_month[i as usize] as DateTimeUnit;
        }
        days += day as DateTimeUnit - 1;
        let seconds = (days * 24 * 60 * 60)
            + (hour as DateTimeUnit * 60 * 60)
            + (minutes as DateTimeUnit * 60)
            + seconds as DateTimeUnit;
        let milliseconds = seconds * 1000;
        Self { milliseconds }
    }

    /// Create a new `DateTime` from a Unix Epoch timestamp (milliseconds).
    /// This is the same as `SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()`.
    /// This is the same as Javascript's `Date.now()`.
    pub fn from_unix_epoch_milliseconds(milliseconds: DateTimeUnit) -> Self {
        let milliseconds = milliseconds + EPOCH_START;
        if milliseconds < 0 {
            panic!("DateTime does not yet support dates before AD 0");
        }
        Self { milliseconds }
    }

    /// Create a new `DateTime` from milliseconds since the start of AD 0.
    pub fn from_milliseconds(milliseconds: DateTimeUnit) -> Self {
        if milliseconds < 0 {
            panic!("DateTime does not yet support dates before AD 0");
        }
        Self { milliseconds }
    }

    pub fn get_milliseconds_of_second(&self) -> u16 {
        (self.milliseconds % 1000) as u16
    }

    pub fn get_seconds_of_minute(&self) -> u8 {
        ((self.milliseconds / 1000) % 60) as u8
    }

    pub fn get_minutes_of_hour(&self) -> u8 {
        ((self.milliseconds / (60 * 1000)) % 60) as u8
    }

    pub fn get_hour_of_day(&self) -> u8 {
        ((self.milliseconds / (60 * 60 * 1000)) % 24) as u8
    }

    pub fn get_day_of_month(&self) -> u16 {
        let year = self.get_year();
        let mut days = self.get_day_of_year() + 1;
        let days_in_month = [
            31u16,
            if is_leap_year(year) { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        let mut month = 0;
        while days > days_in_month[month] {
            days -= days_in_month[month];
            month += 1;
        }
        days
    }

    pub fn get_month(&self) -> u8 {
        let year = self.get_year();
        let day_of_year = self.get_day_of_year();
        match is_leap_year(year) {
            true => match day_of_year {
                1..=31 => 1,
                32..=60 => 2,
                61..=91 => 3,
                92..=121 => 4,
                122..=152 => 5,
                153..=182 => 6,
                183..=213 => 7,
                214..=244 => 8,
                245..=274 => 9,
                275..=305 => 10,
                306..=335 => 11,
                336..=366 => 12,
                _ => 0,
            },
            false => match day_of_year {
                1..=31 => 1,
                32..=59 => 2,
                60..=90 => 3,
                91..=120 => 4,
                121..=151 => 5,
                152..=181 => 6,
                182..=212 => 7,
                213..=243 => 8,
                244..=273 => 9,
                274..=304 => 10,
                305..=334 => 11,
                335..=365 => 12,
                _ => 0,
            },
        }
    }

    pub fn get_day_of_year(&self) -> u16 {
        let mut days = self.to_days();
        let mut year = 0;
        while days > 365 {
            days -= if year > 0 && is_leap_year(year) {
                366
            } else {
                365
            };
            year += 1;
        }
        days as u16
    }

    /// Returns the day of the week, where 0 is Sunday and 6 is Saturday.
    pub fn get_day_of_week(&self) -> u8 {
        const AD_ZERO_OFFSET: i64 = 3;
        let days = self.to_days() + AD_ZERO_OFFSET;
        ((days + 4) % 7) as u8
    }

    pub fn get_year(&self) -> u16 {
        let mut days = self.to_days();
        let mut year = 0;
        while days > 365 {
            days -= if is_leap_year(year) { 366 } else { 365 };
            year += 1;
        }
        year
    }

    pub fn format(&self) -> String {
        time_format(self.milliseconds, "%Y-%m-%d %H:%M:%S.%f")
    }

    /// Returns the total number of milliseconds since the Unix epoch.
    pub fn to_unix_epoch_milliseconds(&self) -> DateTimeUnit {
        self.milliseconds - EPOCH_START
    }

    /// Returns the total number of milliseconds since 0 AD.
    pub fn to_milliseconds(&self) -> DateTimeUnit {
        self.milliseconds
    }

    /// Returns the total number of seconds since 0 AD.
    pub fn to_seconds(&self) -> DateTimeUnit {
        self.milliseconds / 1000
    }

    /// Returns the total number of minutes since 0 AD.
    pub fn to_minutes(&self) -> DateTimeUnit {
        self.milliseconds / (60 * 1000)
    }

    /// Returns the total number of hours since 0 AD.
    pub fn to_hours(&self) -> DateTimeUnit {
        self.milliseconds / (60 * 60 * 1000)
    }

    /// Returns the total number of days since 0 AD, starting at 1.
    pub fn to_days(&self) -> DateTimeUnit {
        self.milliseconds / (24 * 60 * 60 * 1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_formatting_from_new() {
        let datetime = DateTime::new(2023, 5, 28, 14, 36, 46);
        let utc_formatted = "2023-05-28 14:36:46.000";
        assert_eq!(2023, datetime.get_year());
        assert_eq!(5, datetime.get_month());
        assert_eq!(28, datetime.get_day_of_month());
        assert_eq!(14, datetime.get_hour_of_day());
        assert_eq!(36, datetime.get_minutes_of_hour());
        assert_eq!(46, datetime.get_seconds_of_minute());
        assert_eq!(0, datetime.get_day_of_week());
        assert_eq!(utc_formatted, datetime.format());
    }

    #[test]
    fn test_datetime_formatting_pre_1970() {
        let datetime = DateTime::new(1903, 12, 25, 18, 36, 46);
        let utc_formatted = "1903-12-25 18:36:46.000";
        assert_eq!(utc_formatted, datetime.format());
        assert_eq!(1903, datetime.get_year());
        assert_eq!(12, datetime.get_month());
        assert_eq!(25, datetime.get_day_of_month());
        assert_eq!(18, datetime.get_hour_of_day());
        assert_eq!(36, datetime.get_minutes_of_hour());
        assert_eq!(5, datetime.get_day_of_week());
        assert_eq!(46, datetime.get_seconds_of_minute());
    }

    #[test]
    fn test_datetime_formatting_from_milliseconds() {
        let datetime = DateTime::from_unix_epoch_milliseconds(1_685_284_606_076);
        let utc_formatted = "2023-05-28 14:36:46.076";
        assert_eq!(utc_formatted, datetime.format());
        assert_eq!(2023, datetime.get_year());
        assert_eq!(5, datetime.get_month());
        assert_eq!(28, datetime.get_day_of_month());
        assert_eq!(14, datetime.get_hour_of_day());
        assert_eq!(36, datetime.get_minutes_of_hour());
        assert_eq!(0, datetime.get_day_of_week());
        assert_eq!(46, datetime.get_seconds_of_minute());
    }

    // /// Debug test for verifying that the current time is formatted correctly.
    // #[test]
    // fn test_datetime_formatting_from_now() {
    //     let datetime = DateTime::now();
    //     let utc_formatted = "2023-05-28 23:41:00.000";
    //     assert_eq!(2023, datetime.get_year());
    //     assert_eq!(utc_formatted, datetime.format());
    // }
}
