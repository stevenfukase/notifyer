use chrono::{DateTime, Local};

pub fn format_date(date: &DateTime<Local>) -> String {
    date.format("%Y-%m-%dT00:00:00.000+00:00").to_string()
}
