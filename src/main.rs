use lib::{github, slack};
use std::{thread, time};
mod lib;

#[tokio::main]
async fn main() {
    let delay = time::Duration::from_secs(3);
    let contribution_count = &github::todays_contribution_count().await.unwrap_or(0);

    if contribution_count == &0 {
        let message = "You haven't committed today.";
        loop {
            let result = slack::notify(message).await;
            if result.is_ok() {
                println!("Executed");
                break;
            }
            println!("Failed to send Slack");

            thread::sleep(delay);
        }
    } else {
        let message = "You have committed at least once today.";
        loop {
            let result = slack::notify(message).await;
            if result.is_ok() {
                println!("Executed");
                break;
            }
            println!("Failed to send Slack");

            thread::sleep(delay);
        }
    }
}
