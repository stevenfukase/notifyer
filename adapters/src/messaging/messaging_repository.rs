use application::{
    domains::value_objects::date_time::DateTime,
    repositories::messaging_repository_abstract::MessagingRepositoryAbstract,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::{thread, time};
const SLACK_SEND_MESSAGE_ENDPOINT: &str = "https://slack.com/api/chat.postMessage";

#[non_exhaustive]
pub struct MessagingRepository {
    pub slack_channel_id: String,
}



#[async_trait(?Send)]
impl MessagingRepositoryAbstract for MessagingRepository {
    async fn send(&self, blocks: Value, date_time: DateTime) {
        let delay = time::Duration::from_secs(3);

        let perform_request = || async {
            let request_body = json!({
                "channel": env!("SLACK_CHANNEL_ID"),
                "blocks": blocks["blocks"],
            });

            reqwest::Client::new()
                .post(SLACK_SEND_MESSAGE_ENDPOINT)
                .bearer_auth(env!("SLACK_BOT_USER_OAUTH_TOKEN"))
                .json(&request_body)
                .send()
                .await
        };

        loop {
            let result = perform_request().await;
            if result.is_ok() {
                log::info!("Executed");
                break;
            }
            log::error!("Failed to send Slack. Retrying");
            thread::sleep(delay);
        }
    }
}
