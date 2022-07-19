pub mod constants;
pub mod slack_notify;

#[tokio::main]
async fn main() {
    let message = "Hello from rust!";
    slack_notify::slack_notify(message);
}
