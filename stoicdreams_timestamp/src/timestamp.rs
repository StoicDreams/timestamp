use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::time;
use std::time::SystemTime;

/// A time stamp object that stores the time of creation and the time of last update.
/// Intended for use in data storage solutions that support storing complex data structures (e.g. [SurrealDB](https://surrealdb.com/).)
///
/// Expected usage is to use one of the following methods to create a new TimeStamp struct:
/// - `TimeStamp::now()`
/// - `TimeStamp::from_datetime(datetime)`
///
/// Other methods include:
/// - `TimeStamp::update()`
/// - `TimeStamp::time_has_passed_since_last_update(time)`
/// - `TimeStamp::time_has_passed_since_created(time)`
/// - `TimeStamp::get_created()`
/// - `TimeStamp::get_updated()`
///
/// Example:
/// ```
/// use stoicdreams_timestamp::prelude::*;
///
/// let mut timestamp = TimeStamp::now();
/// assert_eq!(timestamp.time_has_passed_since_last_update(Time::from_seconds(1)), false);
/// assert_eq!(timestamp.time_has_passed_since_created(Time::from_seconds(1)), false);
///
/// std::thread::sleep(std::time::Duration::from_secs(1));
///
/// timestamp.update();
/// assert_eq!(timestamp.time_has_passed_since_last_update(Time::from_seconds(1)), false);
/// assert_eq!(timestamp.time_has_passed_since_created(Time::from_seconds(1)), true);
///
/// std::thread::sleep(std::time::Duration::from_secs(1));
/// assert_eq!(timestamp.time_has_passed_since_last_update(Time::from_seconds(1)), true);
///
/// let created_display = timestamp.get_created();
/// let updated_display = timestamp.get_updated();
/// assert_ne!(created_display, updated_display);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct TimeStamp {
    pub created: DateTimeUnit,
    pub updated: DateTimeUnit,
}

/// Returns the current time in milliseconds since the Unix epoch (Midnight of Jan 1st, 1970).
pub fn now_milliseconds() -> DateTimeUnit {
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before Unix epoch");
    now.as_millis() as DateTimeUnit
}

impl Default for TimeStamp {
    fn default() -> Self {
        Self::now()
    }
}

impl TimeStamp {
    pub fn now() -> Self {
        Self {
            created: now_milliseconds(),
            updated: now_milliseconds(),
        }
    }

    pub fn from_datetime(time: DateTime) -> Self {
        Self {
            created: time.to_milliseconds(),
            updated: time.to_milliseconds(),
        }
    }

    pub fn update(&mut self) {
        self.updated = now_milliseconds();
    }

    pub fn time_has_passed_since_last_update(&mut self, time: Time) -> bool {
        let now = now_milliseconds();
        let updated = self.updated;
        let milliseconds = time.to_milliseconds() as DateTimeUnit;
        if updated + milliseconds < now {
            return true;
        }
        false
    }

    pub fn time_has_passed_since_created(&mut self, time: Time) -> bool {
        let now = now_milliseconds();
        let created = self.created;
        let milliseconds = time.to_milliseconds() as DateTimeUnit;
        if created + milliseconds < now {
            return true;
        }
        false
    }

    pub fn get_created(&self) -> String {
        DateTime::from_milliseconds(self.created).format()
    }

    pub fn get_updated(&self) -> String {
        DateTime::from_milliseconds(self.updated).format()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::E;

    use super::*;

    #[test]
    fn test_timestamp_formatting() {
        let timestamp = TimeStamp::from_datetime(DateTime::from_unix_epoch_milliseconds(
            1_685_284_606_076 as DateTimeUnit,
        ));
        let utc_formatted = "2023-05-28 14:36:46.076";
        assert_eq!(1_685_284_606_076, timestamp.created - EPOCH_START);
        assert_eq!(1_685_284_606_076, timestamp.updated - EPOCH_START);
        assert_eq!(utc_formatted, timestamp.get_created());
    }
}
