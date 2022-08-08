use crate::services::{github, slack};

pub async fn send_summary() {
    let todays_contributions = github::get_todays_committed_repo().await;
    println!("{:?}", todays_contributions);
    let contributions_by_repo = todays_contributions.iter();
    let repo_count = contributions_by_repo.count();
    println!("{:?}", repo_count);
    
}
