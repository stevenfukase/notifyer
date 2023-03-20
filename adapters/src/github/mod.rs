pub mod schemas;
pub(super) mod utils;
use chrono::{DateTime, Local};
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

use self::utils::format_date::format_date;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";
const GIT_USERNAME: &str = env!("GIT_USERNAME");

async fn send_github_request<T: Serialize>(
    request_body: &QueryBody<T>,
) -> Result<reqwest::Response, reqwest::Error> {
    let github_token = env!("GIT_ACCESS_TOKEN");
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

pub async fn get_todays_commit_count(date: DateTime<Local>) -> Result<i64, reqwest::Error> {
    let variables = CommitCountVariables {
        login: GIT_USERNAME.to_string(),
        date: format_date(&date),
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

pub async fn get_committed_repos(
    date: DateTime<Local>,
) -> Result<Vec<ContributionsVecByRepo>, reqwest::Error> {
    let variables = CommittedRepoVariables {
        login: GIT_USERNAME.to_string(),
        date: format_date(&date),
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
