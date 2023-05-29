use crate::prelude::*;

pub(crate) const EPOCH_START: DateTimeUnit = 62_167_132_800_000;

pub fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Formats a timestamp in milliseconds since the Unix epoch (Midnight of Jan 1st, 1970) into a
/// human readable format.
/// format options:
/// %Y = year
/// %m = month
/// %D = days
/// %d = day of month
/// %H = hour
/// %M = minute
/// %S = second
/// %f = milliseconds
pub fn time_format_from_unix_epoch(milliseconds_since_epoch: DateTimeUnit, format: &str) -> String {
    time_format(EPOCH_START + milliseconds_since_epoch, format)
}

/// Formats a timestamp in milliseconds since 0 AD into a human readable format.
/// format options:
/// %Y = 4 digit year
/// %m = 2 digit month
/// %D = N digit days when greater than 0
/// %d = 2 digit day of month
/// %H = 2 digit hour
/// %M = 2 digit minute
/// %S = 2 digit second
/// %f = 2 digit milliseconds
pub fn time_format(milliseconds_since_ad_zero: DateTimeUnit, format: &str) -> String {
    let date_time = DateTime::from_milliseconds(milliseconds_since_ad_zero);
    let mut format = format.to_string();
    if format.contains("%Y") {
        let year = date_time.get_year();
        format = format.replace("%Y", &year.to_string());
    }
    if format.contains("%m") {
        let month = date_time.get_month();
        let month = if month < 10 {
            format!("0{}", month)
        } else {
            month.to_string()
        };
        format = format.replace("%m", &month);
    }
    if format.contains("%D") {
        let day = date_time.to_days();
        if day > 0 {
            format = format.replace("%D", &day.to_string());
        } else {
            format = format.replace("%D", "").trim().to_string();
        }
    }
    if format.contains("%d") {
        let day = date_time.get_day_of_month();
        let day = if day < 10 {
            format!("0{}", day)
        } else {
            day.to_string()
        };
        format = format.replace("%d", &day);
    }
    if format.contains("%H") {
        let hour = date_time.get_hour_of_day();
        let hour = if hour < 10 {
            format!("0{}", hour)
        } else {
            hour.to_string()
        };
        format = format.replace("%H", &hour);
    }
    if format.contains("%M") {
        let minute = date_time.get_minutes_of_hour();
        let minute = if minute < 10 {
            format!("0{}", minute)
        } else {
            minute.to_string()
        };
        format = format.replace("%M", &minute);
    }
    if format.contains("%S") {
        let second = date_time.get_seconds_of_minute();
        let second = if second < 10 {
            format!("0{}", second)
        } else {
            second.to_string()
        };
        format = format.replace("%S", &second);
    }
    if format.contains("%f") {
        let millisecond = date_time.get_milliseconds_of_second();
        let millisecond = if millisecond < 10 {
            format!("00{}", millisecond)
        } else if millisecond < 100 {
            format!("0{}", millisecond)
        } else {
            millisecond.to_string()
        };
        format = format.replace("%f", &millisecond);
    }
    format
}

/// Formats a timestamp in nanoseconds since 0 AD into a human readable format.
/// format options:
/// %Y = 4 digit year
/// %m = 2 digit month
/// %D = N digit days when greater than 0
/// %d = 2 digit day of month
/// %H = 2 digit hour
/// %M = 2 digit minute
/// %S = 2 digit second
/// %f = 2 digit nanoseconds
pub fn precise_time_format(nanoseconds: PreciseTimeUnit, format: &str) -> String {
    let milliseconds = (nanoseconds / 1_000_000) as DateTimeUnit;
    let date_time = DateTime::from_milliseconds(milliseconds);
    let mut format = format.to_string();
    if format.contains("%Y") {
        let year = date_time.get_year();
        format = format.replace("%Y", &year.to_string());
    }
    if format.contains("%m") {
        let month = date_time.get_month();
        let month = if month < 10 {
            format!("0{}", month)
        } else {
            month.to_string()
        };
        format = format.replace("%m", &month);
    }
    if format.contains("%D") {
        let day = date_time.to_days();
        if day > 0 {
            format = format.replace("%D", &day.to_string());
        } else {
            format = format.replace("%D", "").trim().to_string();
        }
    }
    if format.contains("%d") {
        let day = date_time.get_day_of_month();
        let day = if day < 10 {
            format!("0{}", day)
        } else {
            day.to_string()
        };
        format = format.replace("%d", &day);
    }
    if format.contains("%H") {
        let hour = date_time.get_hour_of_day();
        let hour = if hour < 10 {
            format!("0{}", hour)
        } else {
            hour.to_string()
        };
        format = format.replace("%H", &hour);
    }
    if format.contains("%M") {
        let minute = date_time.get_minutes_of_hour();
        let minute = if minute < 10 {
            format!("0{}", minute)
        } else {
            minute.to_string()
        };
        format = format.replace("%M", &minute);
    }
    if format.contains("%S") {
        let second = date_time.get_seconds_of_minute();
        let second = if second < 10 {
            format!("0{}", second)
        } else {
            second.to_string()
        };
        format = format.replace("%S", &second);
    }
    if format.contains("%f") {
        let mut nanoseconds_display = (nanoseconds % 1_000_000_000).to_string();
        while nanoseconds_display.len() < 9 {
            nanoseconds_display = format!("0{}", nanoseconds_display);
        }
        format = format.replace("%f", &nanoseconds_display);
    }
    format
}
