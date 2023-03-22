use crate::{
    controllers::{notify::notify, summary::summary, summary_yesterday::summary_yesterday},
    git::git_repository::GitRepository,
    messaging::messaging_service::MessagingService,
};

use super::app_state::AppState;
use std::env;

pub async fn run(
    git_username: &str,
    git_access_token: &str,
    slack_channel_id: &str,
    slack_bot_user_oauth_token: &str,
) {
    let git_repository = GitRepository {};
    let messaging_service = MessagingService::new(slack_channel_id, slack_bot_user_oauth_token);

    let app_state = AppState::new(
        git_username,
        git_access_token,
        git_repository,
        messaging_service,
    );

    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        log::info!("No args passed. Exiting.");
        return;
    }

    if args.contains(&"notify".to_owned()) {
        notify(&app_state).await;
    }

    if args.contains(&"summary".to_owned()) {
        summary_yesterday(&app_state).await;
    }

    if args.contains(&"summary_yesterday".to_owned()) {
        summary(&app_state).await;
    }
}
