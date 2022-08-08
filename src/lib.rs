pub mod features;
pub mod services;
use crate::features::{daily_summary::send_summary, no_commit::notify};

pub async fn run() {
    notify().await;
    send_summary().await;
}
