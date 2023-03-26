use application::domains::{entities::summary_entity::Summary, value_objects::message::Message};
use serde::Serialize;
use serde_json::json;
mod process_plural;
mod you_have_made_count_text;
use process_plural::process_plural;
use you_have_made_count_text::you_have_made_count_text;

pub fn summary(summary_entity: &Summary) -> Message {
    let repo_count = summary_entity.contributions.len() as u32;

    let commit_count = &summary_entity
        .contributions
        .iter()
        .map(|node| node.commit_count)
        .sum();

    #[derive(Debug, Serialize)]
    struct Field {
        r#type: String,
        text: String,
    }

    let commit_fields = &summary_entity
        .contributions
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
    let formatted_date = summary_entity.date_time.to_month_day();

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
