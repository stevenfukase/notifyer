pub mod schemas;
use chrono::{Duration, Local};
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::{header, Client};
use schemas::single_day_committed_repo::{
    single_day_committed_repo::{
        ResponseData as CommittedRepoResponse,
        SingleDayCommittedRepoUserContributionsCollectionCommitContributionsByRepository as ContributionsVecByRepo,
        Variables as CommittedRepoVariables,
    },
    SingleDayCommittedRepo,
};
use schemas::single_day_contributions::{
    single_day_contributions::{
        ResponseData as CommitCountResponse, Variables as CommitCountVariables,
    },
    SingleDayContributions,
};
use serde::Serialize;
use std::iter;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";
const GITHUB_USERNAME: &str = env!("GITHUB_USERNAME");

async fn send_github_request<T: Serialize>(
    request_body: &QueryBody<T>,
) -> Result<reqwest::Response, reqwest::Error> {
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

    // graphql_client::reqwest::post_graphql will cause error
    // when compiling for armv7-unknown-linux-gnueabihf
    let response = client
        .post(GITHUB_ENDPOINT)
        .json(&request_body)
        .send()
        .await?;

    Ok(response)
}

fn generate_date_time(is_yesterday: bool) -> String {
    let mut now = Local::now();
    if is_yesterday {
        now = now - Duration::days(1);
    }
    now.format("%Y-%m-%dT00:00:00.000+00:00").to_string()
}

pub async fn get_todays_commit_count() -> Result<i64, reqwest::Error> {
    let today = generate_date_time(true);
    let variables = CommitCountVariables {
        login: GITHUB_USERNAME.to_string(),
        date: today,
    };

    let request_body = SingleDayContributions::build_query(variables);

    let parsed_response = send_github_request(&request_body)
        .await
        .unwrap()
        .json::<Response<CommitCountResponse>>()
        .await?;

    // TODO: Error handling
    let contributions_data = parsed_response.data.unwrap();
    let user_contributions = contributions_data.user.unwrap().contributions_collection;
    let contributions_count =
        user_contributions.contribution_calendar.weeks[0].contribution_days[0].contribution_count;
    Ok(contributions_count)
}

pub async fn get_committed_repos(is_yesterday: bool) -> Result<Vec<ContributionsVecByRepo>, reqwest::Error> {
    let today = generate_date_time(is_yesterday);
    let variables = CommittedRepoVariables {
        login: GITHUB_USERNAME.to_string(),
        date: today,
    };
    let request_body = SingleDayCommittedRepo::build_query(variables);
    let parsed_response = send_github_request(&request_body)
        .await
        .unwrap()
        .json::<Response<CommittedRepoResponse>>()
        .await?;

    let commit_contributions = parsed_response
        .data
        .unwrap()
        .user
        .unwrap()
        .contributions_collection
        .commit_contributions_by_repository;

    Ok(commit_contributions)
}
