use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::time;
use std::time::SystemTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeStamp {
    pub created: DateTimeUnit,
    pub updated: DateTimeUnit,
}

/// Returns the current time in milliseconds since the Unix epoch (Midnight of Jan 1st, 1970).
pub(crate) fn now() -> DateTimeUnit {
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
            created: now(),
            updated: now(),
        }
    }

    pub fn from_datetime(time: DateTime) -> Self {
        Self {
            created: time.to_milliseconds(),
            updated: time.to_milliseconds(),
        }
    }

    pub fn update(&mut self) {
        self.updated = now();
    }

    pub fn time_has_passed_since_last_update(&mut self, time: Time) -> bool {
        let now = now();
        let updated = self.updated;
        let milliseconds = time.to_milliseconds() as DateTimeUnit;
        if updated + milliseconds < now {
            return true;
        }
        false
    }

    pub fn time_has_passed_since_created(&mut self, time: Time) -> bool {
        let now = now();
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
