use clap::Parser;

use super::app_state::AppState;
use crate::{
    controllers::{notify::notify, summary::summary, summary_yesterday::summary_yesterday},
    git::git_repository::GitRepository,
    messaging::messaging_service::MessagingService,
};
use std::env;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, action)]
    summary: bool,

    #[arg(short, long, action)]
    summary_yesterday: bool,

    #[arg(short, long, action)]
    notify: bool,
}

pub async fn run(
    git_username: &str,
    git_access_token: &str,
    slack_channel_id: &str,
    slack_bot_user_oauth_token: &str,
) {
    let git_repository = GitRepository {
        git_username: git_username.to_string(),
        git_access_token: git_access_token.to_string(),
    };
    let messaging_service = MessagingService {
        slack_channel_id: slack_channel_id.to_string(),
        slack_bot_user_oauth_token: slack_bot_user_oauth_token.to_string(),
    };

    let app_state = AppState {
        git_repository,
        messaging_service,
    };

    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        log::info!("No args passed. Exiting.");
        return;
    }

    if args.contains(&"notify".to_owned()) {
        let result = notify(&app_state).await;
        log::debug!("{:?}", result);
    }

    if args.contains(&"summary".to_owned()) {
        let result = summary(&app_state).await;
        log::debug!("{:?}", result);
    }

    if args.contains(&"summary_yesterday".to_owned()) {
        let result = summary_yesterday(&app_state).await;
        log::debug!("{:?}", result);
    }
}
