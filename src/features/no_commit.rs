use crate::services::{github, slack};
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

    let contribution_count = &github::get_todays_commit_count().await.unwrap_or(0);
    if contribution_count == &0 {
        slack::send(havent_commited_message).await;
    } else {
        println!("Contributed at least once today. not sending notification.");
    }
}
