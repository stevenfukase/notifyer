use crate::{
    domains::enums::application_error::ApplicationError,
    repositories::{
        git_repository_abstract::GitRepositoryAbstract,
        messaging_repository_abstract::MessagingRepositoryAbstract,
    },
};

use super::abstract_usecase::AbstractUsecase;
use async_trait::async_trait;

pub struct NotifyNoCommitUsecase<'a> {
    pub git_repository: &'a dyn GitRepositoryAbstract,
    pub messaging_repository: &'a dyn MessagingRepositoryAbstract,
}

impl<'a> NotifyNoCommitUsecase<'a> {
    pub fn new(
        git_repository: &'a dyn GitRepositoryAbstract,
        messaging_repository: &'a dyn MessagingRepositoryAbstract,
    ) -> Self {
        Self {
            git_repository,
            messaging_repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<String> for NotifyNoCommitUsecase<'a> {
    async fn execute(&self) -> Result<String, ApplicationError> {
        let contribution_count = self.git_repository.get_commit_count(self.date_time);
    }
}

// let havent_commited_message = json!({
//     "blocks": [
//         {
//             "type": "section",
//             "text": {
//               "type": "mrkdwn",
//               "text": "You haven't commited today.".to_owned()
//             }
//         }
//     ]
// });

// let now = Local::now();
// let contribution_count = &github::get_todays_commit_count(now).await.unwrap_or(0);
// log::debug!("Contribution count: {}", contribution_count);

// if contribution_count == &0 {
//     slack::send(havent_commited_message).await;
// } else {
//     log::info!("Contributed at least once today. not sending notification.");
// }
