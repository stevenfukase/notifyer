use crate::services::github::{
    self,
    schemas::single_day_committed_repo::single_day_committed_repo::SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepository as ContributionsByRepo,
};

use serde_json::{json, Value};

pub async fn send_summary() {
    let todays_contributions = github::get_todays_committed_repo().await.unwrap();
    let message_body = create_message_body(&todays_contributions);
    println!("{:?}", todays_contributions);
    let repo_count = todays_contributions.len();
    println!("{:?}", repo_count);
}

fn create_message_body(todays_contributions: &[ContributionsByRepo]) -> Value {
    json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                  "type": "mrkdwn",
                  "text": "You haven't commited today.".to_owned()
                }
            }
        ]
    })
}
