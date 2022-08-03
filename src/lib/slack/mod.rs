use serde_json::{json, Value};
use std::{thread, time};

const SLACK_SEND_MESSAGE_ENDPOINT: &str = "https://slack.com/api/chat.postMessage";

pub async fn perform_request(message_blocks: Value) -> Result<reqwest::Response, reqwest::Error> {
    let request_body = json!({
        "channel": env!("SLACK_CHANNEL_ID"),
        "blocks": message_blocks["blocks"],
    });

    reqwest::Client::new()
        .post(SLACK_SEND_MESSAGE_ENDPOINT)
        .bearer_auth(env!("SLACK_BOT_USER_OAUTH_TOKEN"))
        .json(&request_body)
        .send()
        .await
}

pub async fn send(blocks: Value) {
    let delay = time::Duration::from_secs(3);
    loop {
        let result = perform_request(blocks.clone()).await;
        if result.is_ok() {
            println!("Executed");
            break;
        }
        println!("Failed to send Slack. Retrying");
        thread::sleep(delay);
    }
}
