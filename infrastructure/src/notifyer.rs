use adapters::{
    controllers::{notify::notify, summary::summary, summary_yesterday::summary_yesterday},
    git::git_repository::GitRepository,
    messaging::messaging_service::MessagingService,
    shared::app_state::AppState,
};
use clap::Parser;

#[derive(Parser, Debug)]
enum Args {
    Summary,
    SummaryYesterday,
    Notify,
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

    let args = Args::parse();

    match args {
        Args::Notify => {
            let result = notify(&app_state).await;
            log::debug!("{:?}", result);
        }
        Args::Summary => {
            let result = summary(&app_state).await;
            log::debug!("{:?}", result);
        }
        Args::SummaryYesterday => {
            let result = summary_yesterday(&app_state).await;
            log::debug!("{:?}", result);
        }
    }
}
