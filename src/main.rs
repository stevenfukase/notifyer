use chrono::{DateTime, Local};
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
        .ended_at
        .parse::<DateTime<Local>>()
        .unwrap()
        .date();

    let current_date = Local::now().date();
    let message = "You haven't commited today.";

    if current_date != ended_at {
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
