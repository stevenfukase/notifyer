use crate::{
    domains::{
        entities::contributed_repository::ContributedRepository,
        value_objects::{date_time::DateTime, message::Message},
    },
    utils::messaging::{
        process_plural::process_plural, you_have_made_count_text::you_have_made_count_text,
    },
};
use serde::Serialize;
use serde_json::json;

pub fn create_summary_message_body(
    todays_contributions: &[ContributedRepository],
    date: DateTime,
) -> Message {
    let repo_count = todays_contributions.len() as u32;

    let commit_count = &todays_contributions
        .iter()
        .map(|node| node.commit_count)
        .sum();

    #[derive(Debug, Serialize)]
    struct Field {
        r#type: String,
        text: String,
    }

    let commit_fields = &todays_contributions
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
                    text: process_plural(&node.commit_count, "commit", "commits"),
                },
            ]
        })
        .collect::<Vec<Field>>();

    let subheading = you_have_made_count_text(commit_count, &repo_count);
    let formatted_date = date.formatted();

    let value = json!({
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
    });

    Message::new(value)
}
