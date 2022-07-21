use std::{thread, time};
use lib::{slack, github};
mod lib;

#[tokio::main]
async fn main() {
    let message = "Hello from rustberry!";
    let delay = time::Duration::from_secs(3);

    let github_stat = github::github::get_activity().await;

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
