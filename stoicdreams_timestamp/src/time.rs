use crate::prelude::*;
use serde::{Deserialize, Serialize};

type TimeUnit = u64;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Time {
    milliseconds: TimeUnit,
}

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

    /// Returns the total number of milliseconds since the Unix epoch.
    pub fn to_milliseconds(&self) -> TimeUnit {
        self.milliseconds
    }

    /// Returns the total number of seconds since the Unix epoch.
    pub fn to_seconds(&self) -> TimeUnit {
        self.milliseconds / 1000
    }

    /// Returns the total number of minutes since the Unix epoch.
    pub fn to_minutes(&self) -> TimeUnit {
        self.milliseconds / (60 * 1000)
    }

    /// Returns the total number of hours since the Unix epoch.
    pub fn to_hours(&self) -> TimeUnit {
        self.milliseconds / (60 * 60 * 1000)
    }

    /// Returns the total number of days since the Unix epoch.
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
