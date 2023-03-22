use crate::adapters::{
    github::{
        self,
        schemas::single_day_committed_repo::single_day_committed_repo::{
            SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepository as ContributionsByRepo,
            SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepositoryContributionsNodes as ContributionsNodes,
        },
    },
    slack,
};
use chrono::{DateTime, Duration, Local};
use serde::Serialize;
use serde_json::{json, Value};

pub async fn send_summary(is_yesterday: bool) {
    let date = get_date_time(is_yesterday);
    let todays_contributions = github::get_committed_repos(date).await.unwrap();
    let message_body = create_message_body(&todays_contributions, date);
    slack::send(message_body).await;
}

fn get_date_time(is_yesterday: bool) -> chrono::DateTime<Local> {
    let mut now = Local::now();
    if is_yesterday {
        now = now - Duration::days(1);
    }
    now
}

fn create_message_body(
    todays_contributions: &[ContributionsByRepo],
    date: DateTime<Local>,
) -> Value {
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

    let subheading = you_have_made_count_text(commit_count, repo_count);
    let formatted_date = date.format("%B %e");

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
                    "text": format!("{}\n{}", formatted_date, subheading),
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
