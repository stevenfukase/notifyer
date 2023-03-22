use super::repository::Repository;
use crate::domains::value_objects::date_time::DateTime;

#[non_exhaustive]
pub struct ContributedRepository {
    pub commit_count: u32,
    pub occurred_at: DateTime,
    pub repository: Repository,
}
