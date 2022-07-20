use std::{thread, time};
use utils::slack_notify::slack_notify;
mod utils;

#[tokio::main]
async fn main() {
    let message = "Hello from rustberry!";
    let delay = time::Duration::from_secs(3);

    loop {
        let result = slack_notify(message).await;

        if result.is_ok() {
            println!("Executed");
            break;
        }
        println!("Failed to send Slack");

        thread::sleep(delay);
    }
}
