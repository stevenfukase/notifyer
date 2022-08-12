use crate::services::{
    github::{
        self,
        schemas::single_day_committed_repo::single_day_committed_repo::{
            SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepository as ContributionsByRepo,
            SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepositoryContributionsNodes as ContributionsNodes,
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
    let repo_count: &i64 = &todays_contributions.len().try_into().unwrap_or_default();

    let contributions_nodes = &todays_contributions
        .iter()
        .map(|item| {
            item.contributions.nodes.as_ref().unwrap()[0]
                .as_ref()
                .unwrap()
        })
        .collect::<Vec<&ContributionsNodes>>();

    let commit_count = &contributions_nodes
        .iter()
        .map(|node| node.commit_count)
        .sum();

    struct Field {
        r#type: String,
        text: String,
    }

    let commit_fields = &contributions_nodes.iter().map(|node| {
        println!("{:?}", node);
        json!([
            {
                "type": "mrkdwn",
                "text": "<google.com|stevenfukase/raspberrypi>"
            },
            {
                "type": "mrkdwn",
                "text": "2 commits"
            },
        ])
    });

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
                    "text": you_have_made_count_text(&commit_count, &repo_count),
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

fn you_have_made_count_text(commit_count: &i64, repo_count: &i64) -> String {
    format!(
        "You have made *{}* commits on *{}* repositories",
        commit_count, repo_count
    )
}
