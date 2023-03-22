use crate::domains::{
    entities::contributed_repository::ContributedRepository,
    enums::application_error::ApplicationError,
};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait MessagingRepositoryAbstract {
    async fn send(
        &self,
        todays_contributions: &Vec<ContributedRepository>,
    ) -> Result<(), ApplicationError>;
}
