use application::domains::enums::application_error::ApplicationError;

pub fn summary(
    git_username: &str,
    git_access_token: &str,
    slack_channel_id: &str,
    slack_bot_user_oauth_token: &str,
) -> Result<(), ApplicationError> {
    todo!()
}
