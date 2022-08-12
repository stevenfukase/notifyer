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

use serde::Serialize;
use serde_json::{json, Value};

pub async fn send_summary() {
    let todays_contributions = github::get_todays_committed_repo().await.unwrap();
    let message_body = create_message_body(&todays_contributions);
    slack::send(message_body).await;
}

fn create_message_body(todays_contributions: &[ContributionsByRepo]) -> Value {
    let repo_count: &u64 = &todays_contributions.len().try_into().unwrap_or_default();

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
        .map(|node| node.commit_count as u64)
        .sum();

    #[derive(Debug, Serialize)]
    struct Field {
        r#type: String,
        text: String,
    }

    let commit_fields = &contributions_nodes
        .iter()
        .flat_map(|node| {
            vec![
                Field {
                    r#type: "mrkdwn".to_owned(),
                    text: format!(
                        "<{}|{}>",
                        node.repository.url, node.repository.name_with_owner
                    ),
                },
                Field {
                    r#type: "mrkdwn".to_owned(),
                    text: process_plural(&(node.commit_count as u64), "commit", "commits"),
                },
            ]
        })
        .collect::<Vec<Field>>();

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
                "fields": commit_fields
            },

        ]
    })
}

fn you_have_made_count_text(commit_count: &u64, repo_count: &u64) -> String {
    let commit_plural_text = process_plural(commit_count, "commit", "commits");
    let repo_plural_text = process_plural(repo_count, "repository", "repositories");
    format!(
        "You have made *{}* on *{}*",
        commit_plural_text, repo_plural_text
    )
}

fn process_plural(count: &u64, singular: &str, plural: &str) -> String {
    match count {
        1 => format!("{} {}", count, singular),
        _ => format!("{} {}", count, plural),
    }
}
