use application::domains::value_objects::message::Message;
use serde_json::json;

pub fn notify() -> Message {
    let havent_commited_message = json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                  "type": "mrkdwn",
                  "text": "You haven't commited today.".to_owned()
                }
            }
        ]
    });

    Message::new(havent_commited_message)
}
