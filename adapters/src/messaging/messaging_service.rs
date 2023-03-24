use application::domains::{
    enums::application_error::ApplicationError, value_objects::message::Message,
};
use serde_json::json;
use std::{thread, time};
const SLACK_SEND_MESSAGE_ENDPOINT: &str = "https://slack.com/api/chat.postMessage";

#[non_exhaustive]
pub struct MessagingService {
    pub slack_channel_id: String,
    pub slack_bot_user_oauth_token: String,
}

impl MessagingService {
    pub async fn send(&self, message: Message) -> Result<(), ApplicationError> {
        let delay = time::Duration::from_secs(3);

        let perform_request = || async {
            let request_body = json!({
                "channel": self.slack_channel_id,
                "blocks": message.inner()["blocks"],
            });

            reqwest::Client::new()
                .post(SLACK_SEND_MESSAGE_ENDPOINT)
                .bearer_auth(&*self.slack_bot_user_oauth_token)
                .json(&request_body)
                .send()
                .await
        };

        loop {
            let result = perform_request().await;

            log::debug!("{:?}", result);

            if result.is_ok() {
                log::info!("Executed");
                break;
            }
            log::error!("Failed to send Slack. Retrying");
            thread::sleep(delay);
        }

        Ok(())
    }
}
