mod schemas;
use chrono::Local;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{header, Client};
use std::iter;

use schemas::{
    single_day_commit_repo::single_day_commit_repo,
    single_day_contributions::{
        single_day_contributions::{
            ResponseData as SingleDayContributionsResponse,
            SingleDayContributionsUserContributionsCollection, Variables,
        },
        SingleDayContributions,
    },
};

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

async fn get_single_day_contributions_collection(
    date: String,
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

    let variables = Variables {
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
        .json::<Response<SingleDayContributionsResponse>>()
        .await?;

    // TODO: Error handling
    let contributions_data = parsed_response.data.unwrap();
    let user_contributions = contributions_data.user.unwrap().contributions_collection;

    Ok(user_contributions)
}

pub async fn get_todays_contributions_count() -> Result<i64, reqwest::Error> {
    let today = generate_today_date_time();
    let contributions_collection = get_single_day_contributions_collection(today).await?;
    let contributions_count = contributions_collection.contribution_calendar.weeks[0]
        .contribution_days[0]
        .contribution_count;
    Ok(contributions_count)
}

pub async fn get_todays_contributions() -> Result<i64, reqwest::Error> {
    let today = generate_today_date_time();
    let contributions_collection = get_single_day_contributions_collection(today).await?;
    let contributions_count = contributions_collection.contribution_calendar.weeks[0]
        .contribution_days[0]
        .contribution_count;
    Ok(contributions_count)
}

fn generate_today_date_time() -> String {
    Local::now()
        // TODO: Check if can be omitted
        // .naive_local()
        .format("%Y-%m-%dT00:00:00.000+00:00")
        .to_string()
}
