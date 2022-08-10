use crate::services::{
    github::{
        self,
        schemas::single_day_committed_repo::single_day_committed_repo::{
            SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepository as ContributionsByRepo,
            SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepositoryContributionsNodes as ContributionNodes,
        },
    },
    slack,
};

use serde_json::{json, Value};

pub async fn send_summary() {
    let todays_contributions = github::get_todays_committed_repo().await.unwrap();
    let message_body = create_message_body(&todays_contributions);
    slack::send(message_body).await;
}

fn create_message_body(todays_contributions: &[ContributionsByRepo]) -> Value {
    let repo_count: i64 = todays_contributions.len().try_into().unwrap_or_default();

    let contribution_nodes = todays_contributions
        .iter()
        .map(|item| {
            item.contributions.nodes.as_ref().unwrap()[0]
                .as_ref()
                .unwrap()
        });

    let commit_count = contribution_nodes
        .map(|item| item.commit_count)
        .sum::<i64>();
    // let contribution_repos = todays_contributions.iter().map(|item| {
    //     let node = item.contributions.nodes.as_ref().unwrap()[0]
    //         .as_ref()
    //         .unwrap();
    //     let repository = &node.repository;
    //     println!("{:?}", repository);
    // });

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
                    "text": you_have_made_count_text(commit_count, repo_count),
                }
            },
            {
                "type": "divider"
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*Commits*"
                },
                "fields": [
                    {
                        "type": "mrkdwn",
                        "text": "<google.com|stevenfukase/raspberrypi>"
                    },
                    {
                        "type": "mrkdwn",
                        "text": "2 commits"
                    },
                    {
                        "type": "mrkdwn",
                        "text": "<google.com|stevenfukase/raspberrypi>"
                    },
                    {
                        "type": "mrkdwn",
                        "text": "2 commits"
                    },

                ]
            },

        ]
    })
}

fn you_have_made_count_text(commit_count: i64, repo_count: i64) -> String {
    format!(
        "You have made *{}* commits on *{}* repositories",
        commit_count, repo_count
    )
}
