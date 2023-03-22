use chrono::{Duration, Local};

#[derive(Debug, Clone)]
pub struct DateTime(chrono::DateTime<Local>);

impl DateTime {
    pub fn now() -> Self {
        Self(Local::now())
    }

    pub fn yesterday() -> Self {
        Self(Local::now() - Duration::days(1))
    }

    pub fn to_month_day(&self) -> String {
        self.0.format("%B %e").to_string()
    }

    pub fn to_date_as_utc(&self) -> String {
        self.0.format("%Y-%m-%dT00:00:00.000+00:00").to_string()
    }
}
