use chrono::Local;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{header, Client};
use std::iter;

use self::single_day_contributions::SingleDayContributionsUserContributionsCollection;

type DateTime = String;
type Date = String;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/services/graphql/github_schema.graphql",
    query_path = "src/services/graphql/user_contributions.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct SingleDayContributions;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

async fn get_single_day_contributions_collection(
    date: DateTime,
) -> Result<SingleDayContributionsUserContributionsCollection, reqwest::Error> {
    let github_username = env!("GITHUB_USERNAME");
    let github_token = env!("GITHUB_ACCESS_TOKEN");

    let headers = iter::once((
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", github_token)).unwrap(),
    ))
    .collect();

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(headers)
        .build()?;

    let variables = single_day_contributions::Variables {
        login: github_username.to_string(),
        date,
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

    let contributions_collection = parsed_response
        .data
        .unwrap()
        .user
        .unwrap()
        .contributions_collection;

    Ok(contributions_collection)
}

pub async fn get_todays_contributions_count() -> Result<i64, reqwest::Error> {
    let today = Local::now()
        // TODO: Check if can be omitted
        // .naive_local()
        // .format("%Y-%m-%dT00:00:00.000+00:00");
        .to_string();
    let contributions_collection = get_single_day_contributions_collection(today).await?;
    let contributions_count = contributions_collection.contribution_calendar.weeks[0]
        .contribution_days[0]
        .contribution_count;
    Ok(contributions_count)
}
