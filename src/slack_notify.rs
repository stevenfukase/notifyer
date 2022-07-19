use std::collections::HashMap;

use crate::constants;

pub async fn slack_notify(message: &str) -> Result<reqwest::Response, reqwest::Error> {
    let request_body = HashMap::from([("channel", env!("SLACK_CHANNEL_ID")), ("text", message)]);
    let client = reqwest::Client::new();
    client
        .post(constants::SLACK_SEND_MESSAGE_ENDPOINT)
        .bearer_auth(env!("SLACK_BOT_USER_OAUTH_TOKEN"))
        .json(&request_body)
        .send()
        .await
}
