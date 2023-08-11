use infrastructure::notifyer;

#[tokio::main]
async fn main() {
    env_logger::init();

    let git_username = env!("GIT_USERNAME");
    let git_access_token = env!("GIT_ACCESS_TOKEN");
    let slack_channel_id = env!("SLACK_CHANNEL_ID");
    let slack_bot_user_oauth_token = env!("SLACK_BOT_USER_OAUTH_TOKEN");

    notifyer::run(
        git_username,
        git_access_token,
        slack_channel_id,
        slack_bot_user_oauth_token,
    )
    .await;
}
