use chrono::{Duration, Local};

#[derive(Debug)]
pub struct DateTime(chrono::DateTime<Local>);

impl DateTime {
    pub fn now() -> Self {
        Self(Local::now())
    }

    pub fn yesterday() -> Self {
        let mut now = Local::now() - Duration::days(1);
        Self(now)
    }

    pub fn formatted(&self) -> String {
        self.0.format("%B %e").to_string()
    }
}
