use crate::domains::{
    entities::contributed_repository::ContributedRepository, value_objects::date_time::DateTime,
};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait GitRepositoryAbstract {
    async fn get_committed_repos(
        &self,
        date: DateTime,
    ) -> Result<Vec<ContributedRepository>, Box<dyn std::error::Error>>;
}
