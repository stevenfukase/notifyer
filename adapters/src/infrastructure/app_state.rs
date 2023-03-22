pub struct AppState {
    pub git_username: String,
    pub git_access_token: String,
    pub slack_channel_id: String,
    pub slack_bot_user_oauth_token: String,
    pub messaging_repository: MessagingRepositoryAbstract,
    pub git_repository: GitRepositoryAbstract,
}

impl AppState {
    pub fn new(
        git_username: &str,
        git_access_token: &str,
        slack_channel_id: &str,
        slack_bot_user_oauth_token: &str,
        messaging_repository: MessagingRepositoryAbstract,
        git_repository: GitRepositoryAbstract,
    ) -> Self {
        Self {
            git_username: git_username.to_owned(),
            git_access_token: git_access_token.to_owned(),
            slack_channel_id: slack_channel_id.to_owned(),
            slack_bot_user_oauth_token: slack_bot_user_oauth_token.to_owned(),
            messaging_repository,
            git_repository,
        }
    }
}