use crate::git::git_repository::GitRepository;

#[non_exhaustive]
pub struct AppState {
    pub git_username: String,
    pub git_access_token: String,
    pub slack_channel_id: String,
    pub slack_bot_user_oauth_token: String,
    pub git_repository: GitRepository,
}

impl AppState {
    pub fn new(
        git_username: &str,
        git_access_token: &str,
        slack_channel_id: &str,
        slack_bot_user_oauth_token: &str,
        git_repository: GitRepository,
    ) -> Self {
        Self {
            git_username: git_username.to_string(),
            git_access_token: git_access_token.to_string(),
            slack_channel_id: slack_channel_id.to_string(),
            slack_bot_user_oauth_token: slack_bot_user_oauth_token.to_string(),
            git_repository,
        }
    }
}
