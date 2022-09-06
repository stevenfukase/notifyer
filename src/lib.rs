pub mod features;
pub mod services;
use crate::features::{daily_summary::send_summary, no_commit::notify};
use std::env;

pub async fn run() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        log::info!("No args passed. Exiting.");
        return;
    }

    // TODO: Make more elegant
    if args.contains(&"notify".to_owned()) {
        notify().await;
    }

    if args.contains(&"summary".to_owned()) {
        send_summary(false).await;
    }

    if args.contains(&"summary_yesterday".to_owned()) {
        send_summary(true).await;
    }

    log::warn!("Valid args not found");
}
