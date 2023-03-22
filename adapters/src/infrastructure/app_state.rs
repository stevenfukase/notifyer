use crate::{git::git_repository::GitRepository, messaging::messaging_service::MessagingService};

#[non_exhaustive]
pub struct AppState {
    pub git_username: String,
    pub git_access_token: String,
    pub git_repository: GitRepository,
    pub messaging_service: MessagingService,
}

impl AppState {
    pub fn new(
        git_username: &str,
        git_access_token: &str,
        git_repository: GitRepository,
        messaging_service: MessagingService,
    ) -> Self {
        Self {
            git_username: git_username.to_string(),
            git_access_token: git_access_token.to_string(),
            git_repository,
            messaging_service,
        }
    }
}
