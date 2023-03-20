use crate::{
    domains::enums::application_error::ApplicationError,
    repositories::time_repository_abstract::TimeRepositoryAbstract,
};

use super::abstract_usecase::AbstractUsecase;
use async_trait::async_trait;

#[non_exhaustive]
pub struct NotifySummaryUsecase<'a> {
    pub time_repository: &'a dyn TimeRepositoryAbstract,
    pub is_yesterday: bool,
}

impl<'a> NotifySummaryUsecase<'a> {
    pub fn new(is_yesterday: bool) -> Self {
        Self { is_yesterday }
    }
}

#[async_trait(?Send)]
impl AbstractUsecase<()> for NotifySummaryUsecase {
    async fn execute(&self) -> Result<(), ApplicationError> {
        let date = &self.time_repository(self.is_yesterday);
        let todays_contributions = github::get_committed_repos(date).await.unwrap();
        let message_body = create_message_body(&todays_contributions, date);
        slack::send(message_body).await;
    }
}
