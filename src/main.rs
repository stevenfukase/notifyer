pub mod constants;
pub mod slack_notify;
use slack_notify::slack_notify;

#[tokio::main]
async fn main() {
    let message = "Hello from rustberry!";
    slack_notify(message).await;
}
