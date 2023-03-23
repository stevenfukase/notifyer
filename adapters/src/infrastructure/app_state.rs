use crate::{git::git_repository::GitRepository, messaging::messaging_service::MessagingService};

pub struct AppState {
    pub git_repository: GitRepository,
    pub messaging_service: MessagingService,
}
