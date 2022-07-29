use lib::{github, slack::send_with_retry};
mod lib;

#[tokio::main]
async fn main() {
    let contribution_count = &github::todays_contribution_count().await.unwrap_or(0);

    let message = if contribution_count == &0 {
        "You haven't committed today."
    } else {
        "You have committed at least once today."
    };
    send_with_retry(message).await;
}
