use std::collections::HashMap;

pub mod constants;

#[tokio::main]
async fn main() {
    let message = "Hello from rust!";
    let request_body = HashMap::from([
        ("channel", env!("SLACK_CHANNEL_ID")),
        ("text", message),
    ]);
    let client = reqwest::Client::new();
    let response = client
        .post(constants::SLACK_SEND_MESSAGE_ENDPOINT)
        .bearer_auth(env!("SLACK_BOT_USER_OAUTH_TOKEN"))
        .json(&request_body)
        .send()
        .await;
    println!("{:?}", response);
}
