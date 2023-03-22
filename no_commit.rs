use crate::adapters::{github, slack};
use chrono::Local;
use serde_json::json;

pub async fn notify() {
    let havent_commited_message = json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                  "type": "mrkdwn",
                  "text": "You haven't commited today.".to_owned()
                }
            }
        ]
    });

    let now = Local::now();
    let contribution_count = &github::get_todays_commit_count(now).await.unwrap_or(0);
    log::debug!("Contribution count: {}", contribution_count);

    if contribution_count == &0 {
        slack::send(havent_commited_message).await;
    } else {
        log::info!("Contributed at least once today. not sending notification.");
    }
}