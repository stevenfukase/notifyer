use lib::{github::{self, UserContributions}, slack};
use std::{thread, time};
mod lib;

#[tokio::main]
async fn main() {
    let delay = time::Duration::from_secs(3);
    let result = github::todays_activity_count().await.unwrap().json::<user_contributions::Response>().await;
    println!("{:?}", result);
    

    // if activity_count == &0 {
    //     let message = "You haven't committed today.";
    //     loop {
    //         let result = slack::notify(message).await;
    //         if result.is_ok() {
    //             println!("Executed");
    //             break;
    //         }
    //         println!("Failed to send Slack");

    //         thread::sleep(delay);
    //     }
    // } else {
    //     let message = "You have committed at least once today.";
    //     loop {
    //         let result = slack::notify(message).await;
    //         if result.is_ok() {
    //             println!("Executed");
    //             break;
    //         }
    //         println!("Failed to send Slack");

    //         thread::sleep(delay);
    //     }
    // }
}
