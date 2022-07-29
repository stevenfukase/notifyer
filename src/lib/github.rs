use chrono::{SecondsFormat, Utc};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

type DateTime = String;
type Date = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/graphql/github_schema.graphql",
    query_path = "src/lib/graphql/user_contributions.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct SingleDayContributions;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

pub async fn todays_contribution_count() -> Result<i64, reqwest::Error> {
    let github_username = env!("GITHUB_USERNAME");
    let github_token = env!("GITHUB_ACCESS_TOKEN");

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let variables = single_day_contributions::Variables {
        login: github_username.to_string(),
        date: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    };

    let request_body = SingleDayContributions::build_query(variables);

    // graphql_client::reqwest::post_graphql will cause error
    // when compiling for armv7-unknown-linux-gnueabihf
    let response = client
        .post(GITHUB_ENDPOINT)
        .json(&request_body)
        .send()
        .await?;

    let parsed_response = response
        .json::<Response<single_day_contributions::ResponseData>>()
        .await?;

    let contribution_count = parsed_response
        .data
        .unwrap()
        .user
        .unwrap()
        .contributions_collection
        .contribution_calendar
        .weeks[0]
        .contribution_days[0]
        .contribution_count;

    Ok(contribution_count)
}
