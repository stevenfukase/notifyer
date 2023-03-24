use super::contributed_repository::ContributedRepository;
use crate::domains::value_objects::date_time;

pub struct Summary {
    pub date_time: date_time::DateTime,
    pub contributions: Vec<ContributedRepository>,
}

impl Summary {
    pub fn new(date_time: &date_time::DateTime, contributions: &[ContributedRepository]) -> Self {
        Self {
            date_time: date_time.to_owned(),
            contributions: contributions.to_vec(),
        }
    }
}
