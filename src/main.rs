use lib::{github, slack};
use serde_json::json;
mod lib;

#[tokio::main]
async fn main() {
    let contribution_count = &github::todays_contribution_count().await.unwrap_or(0);

    let message = if contribution_count == &0 {
        json!({
            "blocks": [
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": "You haven't committed today."
                    }
                }
            ]
        })
    } else {
        // has_contribution_message()
        json!({
            "blocks": [
                {
                    "type": "section",
                    "text": {
                      "type": "mrkdwn",
                      "text": you_have_commited_msg(contribution_count)
                    }
                }
            ]
        })
    };
    slack::send(message).await;
}

fn you_have_commited_msg(contribution_count: &i64) -> String {
    let count_string = if contribution_count == &1 {
        "once".to_owned()
    } else if contribution_count == &2 {
        "twice".to_owned()
    } else {
        format!("{} times", contribution_count)
    };
    format!("You have commited {} today.", count_string)
}
