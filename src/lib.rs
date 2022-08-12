pub mod features;
pub mod services;
use crate::features::{daily_summary::send_summary, no_commit::notify};
use std::env;

pub async fn run() {
    let args = env::args().collect::<Vec<String>>();
    // TODO: Make more elegant
    if args.contains(&"notify".to_owned()) {
        notify().await;
    }
    if args.contains(&"summary".to_owned()) {
        send_summary().await;
    }
}
