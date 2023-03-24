use std::str::FromStr;

use chrono::{Duration, Local};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("Unable to parse date_time")]
    ParseError,
}

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

    pub fn to_utc_date(&self) -> String {
        self.0.format("%Y-%m-%dT00:00:00.000+00:00").to_string()
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let parsed_date_time = chrono::DateTime::parse_from_rfc3339(str)
            .map_err(|_error| DateTimeError::ParseError)?;

        Ok(Self(parsed_date_time.into()))
    }
}
