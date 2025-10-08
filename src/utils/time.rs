//! Time utilities
//!
//! This module contains time-related utility functions and helpers.

use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use std::time::SystemTime;

/// Convert system time to UTC DateTime
pub fn system_time_to_utc(time: SystemTime) -> DateTime<Utc> {
    DateTime::from(time)
}

/// Convert system time to local DateTime
pub fn system_time_to_local(time: SystemTime) -> DateTime<Local> {
    DateTime::<Local>::from(system_time_to_utc(time))
}

/// Get current UTC time
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Get current local time
pub fn now_local() -> DateTime<Local> {
    Local::now()
}

/// Convert Unix timestamp to UTC DateTime
pub fn unix_timestamp_to_utc(timestamp: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(timestamp, 0)
}

/// Convert DateTime to Unix timestamp
pub fn datetime_to_unix_timestamp(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp()
}

/// Format duration in a human-readable way
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();

    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        if seconds == 0 {
            format!("{}m", minutes)
        } else {
            format!("{}m {}s", minutes, seconds)
        }
    } else if total_seconds < 86400 {
        let hours = total_seconds / 3600;
        let remaining_seconds = total_seconds % 3600;
        let minutes = remaining_seconds / 60;
        let seconds = remaining_seconds % 60;

        if minutes == 0 && seconds == 0 {
            format!("{}h", hours)
        } else if seconds == 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}h {}m {}s", hours, minutes, seconds)
        }
    } else {
        let days = total_seconds / 86400;
        let remaining_seconds = total_seconds % 86400;
        let hours = remaining_seconds / 3600;

        if hours == 0 {
            format!("{}d", days)
        } else {
            format!("{}d {}h", days, hours)
        }
    }
}

/// Format duration from milliseconds
pub fn format_duration_ms(milliseconds: u64) -> String {
    let duration = Duration::milliseconds(milliseconds as i64);
    format_duration(duration)
}

/// Format duration from ticks (100-nanosecond intervals)
pub fn format_duration_ticks(ticks: u64) -> String {
    let milliseconds = ticks / 10_000; // Convert ticks to milliseconds
    format_duration_ms(milliseconds)
}

/// Convert ticks to seconds
pub fn ticks_to_seconds(ticks: u64) -> f64 {
    ticks as f64 / 10_000_000.0 // 10,000,000 ticks per second
}

/// Convert seconds to ticks
pub fn seconds_to_ticks(seconds: f64) -> u64 {
    (seconds * 10_000_000.0) as u64
}

/// Convert ticks to milliseconds
pub fn ticks_to_milliseconds(ticks: u64) -> u64 {
    ticks / 10_000 // 10,000 ticks per millisecond
}

/// Convert milliseconds to ticks
pub fn milliseconds_to_ticks(milliseconds: u64) -> u64 {
    milliseconds * 10_000
}

/// Format a timestamp in a user-friendly way
pub fn format_timestamp(datetime: DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(datetime);

    if duration.num_seconds() < 60 {
        "Just now".to_string()
    } else if duration.num_minutes() < 60 {
        let minutes = duration.num_minutes();
        if minutes == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", minutes)
        }
    } else if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if duration.num_days() < 7 {
        let days = duration.num_days();
        if days == 1 {
            "Yesterday".to_string()
        } else {
            format!("{} days ago", days)
        }
    } else if duration.num_weeks() < 4 {
        let weeks = duration.num_weeks();
        if weeks == 1 {
            "1 week ago".to_string()
        } else {
            format!("{} weeks ago", weeks)
        }
    } else {
        datetime.format("%B %d, %Y").to_string()
    }
}

/// Parse ISO 8601 datetime string
pub fn parse_iso8601(datetime_str: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(datetime_str)
        .map(|dt| dt.with_timezone(&Utc))
        .ok()
}

/// Format datetime as ISO 8601 string
pub fn format_iso8601(datetime: DateTime<Utc>) -> String {
    datetime.to_rfc3339()
}

/// Check if a datetime is within the last N days
pub fn is_within_days(datetime: DateTime<Utc>, days: i64) -> bool {
    let now = Utc::now();
    let threshold = now - Duration::days(days);
    datetime > threshold
}

/// Get the start of the day for a given datetime
pub fn start_of_day(datetime: DateTime<Local>) -> DateTime<Local> {
    datetime.date_naive().and_hms_opt(0, 0, 0)
        .map(|naive| Local.from_local_datetime(&naive).single())
        .flatten()
        .unwrap_or(datetime)
}

/// Get the end of the day for a given datetime
pub fn end_of_day(datetime: DateTime<Local>) -> DateTime<Local> {
    datetime.date_naive().and_hms_opt(23, 59, 59)
        .map(|naive| Local.from_local_datetime(&naive).single())
        .flatten()
        .unwrap_or(datetime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::seconds(30)), "30s");
        assert_eq!(format_duration(Duration::seconds(90)), "1m 30s");
        assert_eq!(format_duration(Duration::seconds(3661)), "1h 1m 1s");
    }

    #[test]
    fn test_ticks_conversion() {
        assert_eq!(ticks_to_seconds(10_000_000), 1.0);
        assert_eq!(seconds_to_ticks(1.0), 10_000_000);
        assert_eq!(ticks_to_milliseconds(10_000), 1);
        assert_eq!(milliseconds_to_ticks(1), 10_000);
    }
}