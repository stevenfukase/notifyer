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

    pub fn formatted(&self) -> String {
        self.0.format("%B %e").to_string()
    }
}
