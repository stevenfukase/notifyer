use crate::services::{
    github::{
        self,
        schemas::single_day_committed_repo::single_day_committed_repo::SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepository as ContributionsByRepo,
    },
    slack,
};

use serde_json::{json, Value};

pub async fn send_summary() {
    let todays_contributions = github::get_todays_committed_repo().await.unwrap();
    let message_body = create_message_body(&todays_contributions);
    println!("{:?}", todays_contributions);
    let repo_count = todays_contributions.len();
    println!("{:?}", repo_count);
    slack::send(message_body).await;
}

fn create_message_body(todays_contributions: &[ContributionsByRepo]) -> Value {
    let repo_count = todays_contributions.len();

    json!({
        "blocks": [
            {
                "type": "header",
                "text": {
                    "type": "plain_text",
                    "text": ":computer: Activity Report",
                    "emoji": true
                }
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    // "text": you_have_made_count_text(repo_count, ),
                }
            },
            {
                "type": "divider"
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*Commits*\n<google.com|stevenfukase/raspberrypi>\t2 commits\n<google.com|stevenfukase/actix-playground>\t2 commits"
                }
            },
   
        ]
    })
}

// fn you_have_made_count_text(commit_count: i64, repo_count: i64) -> String {

// }

// {
// 	"blocks": [
// 		{
// 			"type": "header",
// 			"text": {
// 				"type": "plain_text",
// 				"text": ":computer: Activity Report",
// 				"emoji": true
// 			}
// 		},
// 		{
// 			"type": "section",
// 			"text": {
// 				"type": "mrkdwn",
// 				"text": "You have made *6* contributions on *3* repositories"
// 			}
// 		},
// 		{
// 			"type": "divider"
// 		},
// 		{
// 			"type": "section",
// 			"text": {
// 				"type": "mrkdwn",
// 				"text": "*Commits*\n<google.com|stevenfukase/raspberrypi>\t2 commits\n<google.com|stevenfukase/actix-playground>\t2 commits"
// 			}
// 		},
// 		{
// 			"type": "divider"
// 		},
// 		{
// 			"type": "section",
// 			"text": {
// 				"type": "mrkdwn",
// 				"text": "*Pull Requests*\n<google.com|stevenfukase/raspberrypi>\n<google.com|stevenfukase/actix-playground>"
// 			}
// 		}
// 	]
// }
