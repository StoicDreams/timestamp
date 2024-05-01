use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub type TimeUnit = u64;

/// A time struct that can be used to represent a time in milliseconds.
/// Expected usage is to use one of the following methods to create a new Time struct:
/// - `Time::new(days, hours, minutes, seconds)`
/// - `Time::from_days(days)`
/// - `Time::from_hours(hours)`
/// - `Time::from_minutes(minutes)`
/// - `Time::from_seconds(seconds)`
/// - `Time::from_milliseconds(milliseconds)`
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = Time::new(0, 0, 0, 0);
/// assert_eq!(time.get_milliseconds_of_second(), 0);
/// assert_eq!(time.get_seconds_of_minute(), 0);
/// assert_eq!(time.get_minutes_of_hour(), 0);
/// assert_eq!(time.get_hour_of_day(), 0);
/// assert_eq!(time.format(), "00:00:00.000");
///```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = Time::from_days(1);
/// assert_eq!(time.get_milliseconds_of_second(), 0);
/// assert_eq!(time.get_seconds_of_minute(), 0);
/// assert_eq!(time.get_minutes_of_hour(), 0);
/// assert_eq!(time.get_hour_of_day(), 0);
/// assert_eq!(time.format(), "1 00:00:00.000");
///```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = Time::from_hours(1);
/// assert_eq!(time.get_milliseconds_of_second(), 0);
/// assert_eq!(time.get_seconds_of_minute(), 0);
/// assert_eq!(time.get_minutes_of_hour(), 0);
/// assert_eq!(time.get_hour_of_day(), 1);
/// assert_eq!(time.format(), "01:00:00.000");
///```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = Time::from_minutes(1);
/// assert_eq!(time.get_milliseconds_of_second(), 0);
/// assert_eq!(time.get_seconds_of_minute(), 0);
/// assert_eq!(time.get_minutes_of_hour(), 1);
/// assert_eq!(time.get_hour_of_day(), 0);
/// assert_eq!(time.format(), "00:01:00.000");
///```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = Time::from_seconds(1);
/// assert_eq!(time.get_milliseconds_of_second(), 0);
/// assert_eq!(time.get_seconds_of_minute(), 1);
/// assert_eq!(time.get_minutes_of_hour(), 0);
/// assert_eq!(time.get_hour_of_day(), 0);
/// assert_eq!(time.format(), "00:00:01.000");
///```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = Time::from_milliseconds(1);
/// assert_eq!(time.get_milliseconds_of_second(), 1);
/// assert_eq!(time.get_seconds_of_minute(), 0);
/// assert_eq!(time.get_minutes_of_hour(), 0);
/// assert_eq!(time.get_hour_of_day(), 0);
/// assert_eq!(time.format(), "00:00:00.001");
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Time {
    milliseconds: TimeUnit,
}

#[cfg(feature = "sqlx")]
impl sqlx::FromRow for Time {}

impl Time {
    pub fn new(days: u16, hours: u8, minutes: u8, seconds: u8) -> Self {
        let seconds = (days as TimeUnit * 24 * 60 * 60)
            + (hours as TimeUnit * 60 * 60)
            + (minutes as TimeUnit * 60)
            + seconds as TimeUnit;
        let milliseconds = seconds * 1000;
        Self { milliseconds }
    }

    pub fn from_days(days: u16) -> Self {
        let hours = days as TimeUnit * 24;
        let minutes = hours * 60;
        let seconds = minutes * 60;
        let milliseconds = seconds * 1000;
        Self { milliseconds }
    }

    pub fn from_hours(hours: TimeUnit) -> Self {
        let minutes = hours * 60;
        let seconds = minutes * 60;
        let milliseconds = seconds * 1000;
        Self { milliseconds }
    }

    pub fn from_minutes(minutes: TimeUnit) -> Self {
        let seconds = minutes * 60;
        let milliseconds = seconds * 1000;
        Self { milliseconds }
    }

    pub fn from_seconds(seconds: TimeUnit) -> Self {
        let milliseconds = seconds * 1000;
        Self { milliseconds }
    }

    pub fn from_milliseconds(milliseconds: TimeUnit) -> Self {
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

    pub fn format(&self) -> String {
        time_format(self.milliseconds as DateTimeUnit, "%D %H:%M:%S.%f")
    }

    /// Returns the total number of milliseconds.
    pub fn to_milliseconds(&self) -> TimeUnit {
        self.milliseconds
    }

    /// Returns the total number of seconds.
    pub fn to_seconds(&self) -> TimeUnit {
        self.milliseconds / 1000
    }

    /// Returns the total number of minutes.
    pub fn to_minutes(&self) -> TimeUnit {
        self.milliseconds / (60 * 1000)
    }

    /// Returns the total number of hours.
    pub fn to_hours(&self) -> TimeUnit {
        self.milliseconds / (60 * 60 * 1000)
    }

    /// Returns the total number of days.
    pub fn to_days(&self) -> TimeUnit {
        self.milliseconds / (24 * 60 * 60 * 1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_formatting_from_zero() {
        let time = Time::from_milliseconds(0);
        let utc_formatted = "00:00:00.000";
        assert_eq!(utc_formatted, time.format());
        assert_eq!(0, time.get_hour_of_day());
        assert_eq!(0, time.get_minutes_of_hour());
        assert_eq!(0, time.get_seconds_of_minute());
    }

    #[test]
    fn test_time_formatting() {
        let time = Time::from_milliseconds(5_504_294_967_295);
        let utc_formatted = "63707 02:49:27.295";
        assert_eq!(utc_formatted, time.format());
        assert_eq!(2, time.get_hour_of_day());
        assert_eq!(49, time.get_minutes_of_hour());
        assert_eq!(27, time.get_seconds_of_minute());
    }
}
