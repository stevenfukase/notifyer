pub mod features;
pub mod services;
use crate::features::no_commit::notify;

pub async fn run() {
    notify().await;
}
