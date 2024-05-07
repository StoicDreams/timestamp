use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub type StopWatchUnit = u128;

/// A stopwatch that can be used to measure the ellapsed time between two points in time - Measured in nanoseconds.
///
/// Expected usage is to use one of the following methods to create a new StopWatch struct:
/// - `StopWatch::start()`
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let stopwatch = StopWatch::start();
/// std::thread::sleep(std::time::Duration::from_millis(1));
/// // assert_ne!(stopwatch.ellapsed_nanoseconds(), 0);
/// // assert_ne!(stopwatch.ellapsed_microseconds(), 0);
/// assert_ne!(stopwatch.ellapsed_milliseconds(), 0);
/// assert_eq!(stopwatch.ellapsed_seconds(), 0);
/// assert_eq!(stopwatch.ellapsed_minutes(), 0);
/// assert_eq!(stopwatch.ellapsed_hours(), 0);
/// assert_eq!(stopwatch.ellapsed_days(), 0);
/// let ellapsed = stopwatch.ellapsed();
/// println!("{}", ellapsed.format());
/// let utc_formatted = "00:00:00.000000000";
/// assert_ne!(utc_formatted, ellapsed.format());
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StopWatch {
    start: StopWatchUnit,
}

/// Returns the current time in nanoseconds since start (Midnight of Jan 1st, 1970).
pub fn now_nanoseconds() -> StopWatchUnit {
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before Unix epoch");
    now.as_nanos() as StopWatchUnit
}

impl StopWatch {
    pub fn start() -> Self {
        let start = now_nanoseconds();
        Self { start }
    }

    pub fn ellapsed(&self) -> PreciseTime {
        PreciseTime::from_nanoseconds(self.ellapsed_nanoseconds())
    }

    /// Returns the total number of nanoseconds since start.
    pub fn ellapsed_nanoseconds(&self) -> StopWatchUnit {
        now_nanoseconds() - self.start
    }

    /// Returns the total number of microseconds since start.
    pub fn ellapsed_microseconds(&self) -> StopWatchUnit {
        (now_nanoseconds() - self.start) / 1000
    }

    /// Returns the total number of milliseconds since start.
    pub fn ellapsed_milliseconds(&self) -> StopWatchUnit {
        (now_nanoseconds() - self.start) / 1000000
    }

    /// Returns the total number of seconds since start.
    pub fn ellapsed_seconds(&self) -> StopWatchUnit {
        (now_nanoseconds() - self.start) / 1000000000
    }

    /// Returns the total number of minutes since start.
    pub fn ellapsed_minutes(&self) -> StopWatchUnit {
        (now_nanoseconds() - self.start) / (60 * 1000000000)
    }

    /// Returns the total number of hours since start.
    pub fn ellapsed_hours(&self) -> StopWatchUnit {
        (now_nanoseconds() - self.start) / (60 * 60 * 1000000000)
    }

    /// Returns the total number of days since start.
    pub fn ellapsed_days(&self) -> StopWatchUnit {
        (now_nanoseconds() - self.start) / (24 * 60 * 60 * 1000000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopwatch_expecting_some_nanoseconds_to_pass() {
        let time = StopWatch::start();
        let ellapsed = time.ellapsed();
        let utc_formatted = "00:00:00.000000000";
        assert_ne!(utc_formatted, ellapsed.format());
    }
}
