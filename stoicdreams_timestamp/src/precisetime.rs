use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub type PreciseTimeUnit = u128;

/// A precise time struct that can be used to represent a time in nanoseconds.
/// Expected usage is to use one of the following methods to create a new PreciseTime struct:
/// - `PreciseTime::new(days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds)`
/// - `PreciseTime::from_nanoseconds(nanoseconds)`
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::from_nanoseconds(1);
/// assert_eq!(time.to_nanoseconds(), 1);
/// assert_eq!(time.to_microseconds(), 0);
/// assert_eq!(time.to_milliseconds(), 0);
/// assert_eq!(time.to_seconds(), 0);
/// assert_eq!(time.to_minutes(), 0);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:00:00.000000001");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 0, 0, 0, 0, 0, 0);
/// assert_eq!(time.to_nanoseconds(), 0);
/// assert_eq!(time.to_microseconds(), 0);
/// assert_eq!(time.to_milliseconds(), 0);
/// assert_eq!(time.to_seconds(), 0);
/// assert_eq!(time.to_minutes(), 0);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:00:00.000000000");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 0, 0, 0, 0, 0, 1);
/// assert_eq!(time.to_nanoseconds(), 1);
/// assert_eq!(time.to_microseconds(), 0);
/// assert_eq!(time.to_milliseconds(), 0);
/// assert_eq!(time.to_seconds(), 0);
/// assert_eq!(time.to_minutes(), 0);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:00:00.000000001");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 0, 0, 0, 0, 1, 0);
/// assert_eq!(time.to_nanoseconds(), 1000);
/// assert_eq!(time.to_microseconds(), 1);
/// assert_eq!(time.to_milliseconds(), 0);
/// assert_eq!(time.to_seconds(), 0);
/// assert_eq!(time.to_minutes(), 0);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:00:00.000001000");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 0, 0, 0, 1, 0, 0);
/// assert_eq!(time.to_nanoseconds(), 1000000);
/// assert_eq!(time.to_microseconds(), 1000);
/// assert_eq!(time.to_milliseconds(), 1);
/// assert_eq!(time.to_seconds(), 0);
/// assert_eq!(time.to_minutes(), 0);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:00:00.001000000");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 0, 0, 1, 0, 0, 0);
/// assert_eq!(time.to_nanoseconds(), 1000000000);
/// assert_eq!(time.to_microseconds(), 1000000);
/// assert_eq!(time.to_milliseconds(), 1000);
/// assert_eq!(time.to_seconds(), 1);
/// assert_eq!(time.to_minutes(), 0);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:00:01.000000000");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 0, 1, 0, 0, 0, 0);
/// assert_eq!(time.to_nanoseconds(), 60000000000);
/// assert_eq!(time.to_microseconds(), 60000000);
/// assert_eq!(time.to_milliseconds(), 60000);
/// assert_eq!(time.to_seconds(), 60);
/// assert_eq!(time.to_minutes(), 1);
/// assert_eq!(time.to_hours(), 0);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "00:01:00.000000000");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(0, 1, 0, 0, 0, 0, 0);
/// assert_eq!(time.to_nanoseconds(), 3600000000000);
/// assert_eq!(time.to_microseconds(), 3600000000);
/// assert_eq!(time.to_milliseconds(), 3600000);
/// assert_eq!(time.to_seconds(), 3600);
/// assert_eq!(time.to_minutes(), 60);
/// assert_eq!(time.to_hours(), 1);
/// assert_eq!(time.to_days(), 0);
/// assert_eq!(time.format(), "01:00:00.000000000");
/// ```
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let time = PreciseTime::new(1, 0, 0, 0, 0, 0, 0);
/// assert_eq!(time.to_nanoseconds(), 86400000000000);
/// assert_eq!(time.to_microseconds(), 86400000000);
/// assert_eq!(time.to_milliseconds(), 86400000);
/// assert_eq!(time.to_seconds(), 86400);
/// assert_eq!(time.to_minutes(), 1440);
/// assert_eq!(time.to_hours(), 24);
/// assert_eq!(time.to_days(), 1);
/// assert_eq!(time.format(), "1 00:00:00.000000000");
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct PreciseTime {
    nanoseconds: PreciseTimeUnit,
}

impl PreciseTime {
    pub fn new(
        days: u16,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
        microseconds: u16,
        nanoseconds: u16,
    ) -> Self {
        let nanoseconds = nanoseconds as PreciseTimeUnit
            + (microseconds as PreciseTimeUnit * 1000)
            + (milliseconds as PreciseTimeUnit * 1000 * 1000)
            + (seconds as PreciseTimeUnit * 1000 * 1000 * 1000)
            + (minutes as PreciseTimeUnit * 60 * 1000 * 1000 * 1000)
            + (hours as PreciseTimeUnit * 60 * 60 * 1000 * 1000 * 1000)
            + (days as PreciseTimeUnit * 24 * 60 * 60 * 1000 * 1000 * 1000);
        Self { nanoseconds }
    }

    pub fn from_nanoseconds(nanoseconds: PreciseTimeUnit) -> Self {
        Self { nanoseconds }
    }

    pub fn format(&self) -> String {
        precise_time_format(self.nanoseconds, "%D %H:%M:%S.%f")
    }

    /// Returns the total number of nanoseconds.
    pub fn to_nanoseconds(&self) -> PreciseTimeUnit {
        self.nanoseconds
    }

    /// Returns the total number of microseconds.
    pub fn to_microseconds(&self) -> PreciseTimeUnit {
        self.nanoseconds / 1000
    }

    /// Returns the total number of milliseconds.
    pub fn to_milliseconds(&self) -> PreciseTimeUnit {
        self.nanoseconds / 1000000
    }

    /// Returns the total number of seconds.
    pub fn to_seconds(&self) -> PreciseTimeUnit {
        self.nanoseconds / 1000000000
    }

    /// Returns the total number of minutes.
    pub fn to_minutes(&self) -> PreciseTimeUnit {
        self.nanoseconds / (60 * 1000000000)
    }

    /// Returns the total number of hours.
    pub fn to_hours(&self) -> PreciseTimeUnit {
        self.nanoseconds / (60 * 60 * 1000000000)
    }

    /// Returns the total number of days.
    pub fn to_days(&self) -> PreciseTimeUnit {
        self.nanoseconds / (24 * 60 * 60 * 1000000000)
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
