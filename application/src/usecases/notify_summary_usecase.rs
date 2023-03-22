use super::abstract_usecase::AbstractUsecase;
use crate::{
    domains::{
        entities::contributed_repository::ContributedRepository,
        enums::application_error::ApplicationError, value_objects::date_time::DateTime,
    },
    repositories::{
        git_repository_abstract::GitRepositoryAbstract,
        messaging_repository_abstract::MessagingRepositoryAbstract,
    },
};
use async_trait::async_trait;

#[non_exhaustive]
pub struct NotifySummaryUsecase<'a> {
    pub git_repository: &'a dyn GitRepositoryAbstract,
    pub messaging_repository: &'a dyn MessagingRepositoryAbstract,
    pub date: DateTime,
}

impl<'a> NotifySummaryUsecase<'a> {
    pub fn new(
        git_repository: &'a dyn GitRepositoryAbstract,
        messaging_repository: &'a dyn MessagingRepositoryAbstract,
        date: DateTime,
    ) -> Self {
        Self {
            git_repository,
            messaging_repository,
            date,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<Vec<ContributedRepository>> for NotifySummaryUsecase<'a> {
    async fn execute(&self) -> Result<Vec<ContributedRepository>, ApplicationError> {
        self.git_repository.get_committed_repos(&self.date).await
    }
}
