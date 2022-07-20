use std::collections::HashMap;

const SLACK_SEND_MESSAGE_ENDPOINT: &str = "https://slack.com/api/chat.postMessage";

pub async fn notify(message: &str) -> Result<reqwest::Response, reqwest::Error> {
    let request_body = HashMap::from([("channel", env!("SLACK_CHANNEL_ID")), ("text", message)]);
    let client = reqwest::Client::new();
    client
        .post(SLACK_SEND_MESSAGE_ENDPOINT)
        .bearer_auth(env!("SLACK_BOT_USER_OAUTH_TOKEN"))
        .json(&request_body)
        .send()
        .await
}
