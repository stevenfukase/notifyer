mod interfaces;
mod lib;
use interfaces::message;
use lib::{github, slack};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let contribution_count = &github::todays_contribution_count().await.unwrap_or(0);
    let message = create_msg_blocks(contribution_count).to_string();
    // slack::send(message).await;
    message::Message::send(message);
}

fn create_msg_blocks(contribution_count: &i64) -> Value {
    json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                  "type": "mrkdwn",
                  "text": todays_contribution_msg(contribution_count)
                }
            }
        ]
    })
}

fn todays_contribution_msg(contribution_count: &i64) -> String {
    match contribution_count {
        0 => "You haven't commited today.".to_owned(),
        1 => "You have commited once today.".to_owned(),
        2 => "You have commited twice today.".to_owned(),
        _ => format!("You have commited {} times today.", contribution_count),
    }
}
