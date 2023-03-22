use application::repositories::messaging_repository_abstract::MessagingRepositoryAbstract;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::{thread, time};

const SLACK_SEND_MESSAGE_ENDPOINT: &str = "https://slack.com/api/chat.postMessage";

pub struct MessagingRepository {
    pub app_state: AppState,
}

#[async_trait(?Send)]
impl MessagingRepositoryAbstract for MessagingRepository {
    async fn send(&self, blocks: Value) {
        let delay = time::Duration::from_secs(3);

        loop {
            let result = perform_request(blocks.clone()).await;
            if result.is_ok() {
                log::info!("Executed");
                break;
            }
            log::error!("Failed to send Slack. Retrying");
            thread::sleep(delay);
        }
    }
}

async fn perform_request(message_blocks: Value) -> Result<reqwest::Response, reqwest::Error> {
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
