use adapters::controllers::{
    notify::notify, summary::summary, summary_yesterday::summary_yesterday,
};
use std::env;

pub async fn run(
    git_username: &str,
    git_access_token: &str,
    slack_channel_id: &str,
    slack_bot_user_oauth_token: &str,
) {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        log::info!("No args passed. Exiting.");
        return;
    }

    if args.contains(&"notify".to_owned()) {
        notify(
            git_username,
            git_access_token,
            slack_channel_id,
            slack_bot_user_oauth_token,
        )
        .await;
    }

    if args.contains(&"summary".to_owned()) {
        summary_yesterday(
            git_username,
            git_access_token,
            slack_channel_id,
            slack_bot_user_oauth_token,
        )
        .await;
    }

    if args.contains(&"summary_yesterday".to_owned()) {
        summary(
            git_username,
            git_access_token,
            slack_channel_id,
            slack_bot_user_oauth_token,
        )
        .await;
    }
}
