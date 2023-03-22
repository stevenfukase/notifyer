use crate::{
    domains::enums::application_error::ApplicationError,
    repositories::{
        git_repository_abstract::GitRepositoryAbstract,
        messaging_repository_abstract::MessagingRepositoryAbstract,
        time_repository_abstract::TimeRepositoryAbstract,
    },
};

use super::abstract_usecase::AbstractUsecase;
use async_trait::async_trait;

#[non_exhaustive]
pub struct NotifySummaryUsecase<'a> {
    pub time_repository: &'a dyn TimeRepositoryAbstract,
    pub git_repository: &'a dyn GitRepositoryAbstract,
    pub message_repository: &'a dyn MessagingRepositoryAbstract,
    pub is_yesterday: bool,
}

impl<'a> NotifySummaryUsecase<'a> {
    pub fn new(
        time_repository: &'a dyn TimeRepositoryAbstract,
        git_repository: &'a dyn GitRepositoryAbstract,
        message_repository: &'a dyn MessagingRepositoryAbstract,
        is_yesterday: bool,
    ) -> Self {
        Self { is_yesterday }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<()> for NotifySummaryUsecase<'a> {
    async fn execute(&self) -> Result<(), ApplicationError> {
        let date = &self.time_repository(self.is_yesterday);
        let todays_contributions = &self.git_repository.get_committed_repos(date).await.unwrap();
        let message_body = create_message_body(&todays_contributions, date);
        &self.message_repository.send(message_body).await;
    }
}
