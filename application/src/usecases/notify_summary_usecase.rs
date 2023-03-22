use crate::{
    domains::{enums::application_error::ApplicationError, value_objects::date_time::DateTime},
    repositories::{
        git_repository_abstract::GitRepositoryAbstract,
        messaging_repository_abstract::MessagingRepositoryAbstract,
    },
};

use super::abstract_usecase::AbstractUsecase;
use async_trait::async_trait;

#[non_exhaustive]
pub struct NotifySummaryUsecase<'a> {
    pub git_repository: &'a dyn GitRepositoryAbstract,
    pub messaging_repository: &'a dyn MessagingRepositoryAbstract,
    pub is_yesterday: bool,
}

impl<'a> NotifySummaryUsecase<'a> {
    pub fn new(
        git_repository: &'a dyn GitRepositoryAbstract,
        messaging_repository: &'a dyn MessagingRepositoryAbstract,
        is_yesterday: bool,
    ) -> Self {
        Self {
            git_repository,
            messaging_repository,
            is_yesterday,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<()> for NotifySummaryUsecase<'a> {
    async fn execute(&self) -> Result<(), ApplicationError> {
        let date = if self.is_yesterday {
            DateTime::yesterday()
        } else {
            DateTime::now()
        };

        let todays_contributions = self
            .git_repository
            .get_committed_repos(&date)
            .await?;

        self.messaging_repository.send(&todays_contributions, date).await
    }
}
