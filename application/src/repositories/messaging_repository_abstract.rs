use crate::domains::{
    entities::contributed_repository::ContributedRepository,
    enums::application_error::ApplicationError, value_objects::date_time::DateTime,
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
        date: DateTime,
    ) -> Result<(), ApplicationError>;
}
