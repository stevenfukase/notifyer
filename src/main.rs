use lib::{github, slack};
use std::{thread, time};
mod lib;

#[tokio::main]
async fn main() {
    let delay = time::Duration::from_secs(3);
    let github_stat = github::get_activity().await;

    let message = format!("{:?}", github_stat);
    loop {
        let result = slack::notify(&message).await;

        if result.is_ok() {
            println!("Executed");
            break;
        }
        println!("Failed to send Slack");

        thread::sleep(delay);
    }
}
