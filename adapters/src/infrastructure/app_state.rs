use crate::git::git_repository::GitRepository;
use crate::messaging::messaging_repository::MessagingRepository;

pub struct AppState {
    pub git_username: String,
    pub git_access_token: String,
    pub slack_channel_id: String,
    pub slack_bot_user_oauth_token: String,
    pub messaging_repository: MessagingRepository,
    pub git_repository: GitRepository,
}
