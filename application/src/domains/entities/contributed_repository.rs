use super::repository::Repository;
use crate::domains::value_objects::date_time::DateTime;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ContributedRepository {
    pub commit_count: u32,
    pub occurred_at: DateTime,
    pub repository: Repository,
}

impl ContributedRepository {
    pub fn new(
        commit_count: &u32,
        occurred_at: &DateTime,
        name_with_owner: &str,
        url: &str,
        open_graph_image_url: &str,
    ) -> Self {
        let repository = Repository::new(name_with_owner, url, open_graph_image_url);
        Self {
            commit_count: *commit_count,
            occurred_at: occurred_at.clone(),
            repository,
        }
    }
}
