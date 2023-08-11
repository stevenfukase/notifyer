use application::domains::{
    enums::application_error::ApplicationError, value_objects::message::Message,
};
use serde_json::json;
const SLACK_SEND_MESSAGE_ENDPOINT: &str = "https://slack.com/api/chat.postMessage";

pub struct MessagingService {
    pub slack_channel_id: String,
    pub slack_bot_user_oauth_token: String,
}

impl MessagingService {
    pub async fn send(&self, message: Message) -> Result<(), ApplicationError> {
        let request_body = json!({
            "channel": self.slack_channel_id,
            "blocks": message.inner()["blocks"],
        });

        let result = reqwest::Client::new()
            .post(SLACK_SEND_MESSAGE_ENDPOINT)
            .bearer_auth(&*self.slack_bot_user_oauth_token)
            .json(&request_body)
            .send()
            .await;

        log::debug!("{:?}", result);

        Ok(())
    }
}
