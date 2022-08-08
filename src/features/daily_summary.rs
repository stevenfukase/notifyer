use crate::services::{github, slack};

pub async fn send_summary() {
    let todays_contributions = github::get_todays_committed_repo().await;
}
