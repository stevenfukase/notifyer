use lib::{github, slack};
use std::{thread, time};
mod lib;

#[tokio::main]
async fn main() {
    let delay = time::Duration::from_secs(3);
    let ended_at = github::get_activity()
        .await
        .unwrap()
        .data
        .unwrap()
        .user
        .unwrap()
        .contributions_collection
        .ended_at;

    let message = format!("{:?}", ended_at);
    println!("{:?}", message);

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
