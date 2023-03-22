use crate::domains::{
    entities::contributed_repository::ContributedRepository,
    enums::application_error::ApplicationError, value_objects::date_time::DateTime,
};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait GitRepositoryAbstract {
    async fn get_committed_repos(
        &self,
        date: &DateTime,
    ) -> Result<Vec<ContributedRepository>, ApplicationError>;

    async fn get_commit_count(&self, date: &DateTime) -> Result<u32, ApplicationError>;
}
