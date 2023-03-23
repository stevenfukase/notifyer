use super::schemas::single_day_committed_repo::{
    single_day_committed_repo::{
        ResponseData as CommittedRepoResponse, Variables as CommittedRepoVariables,
    },
    SingleDayCommittedRepo,
};
use super::schemas::single_day_contributions::{
    single_day_contributions::{
        ResponseData as CommitCountResponse, Variables as CommitCountVariables,
    },
    SingleDayContributions,
};
use application::{
    domains::{
        entities::contributed_repository::ContributedRepository,
        enums::application_error::ApplicationError, value_objects::date_time::DateTime,
    },
    repositories::git_repository_abstract::GitRepositoryAbstract,
};
use async_trait::async_trait;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::{header, Client};
use serde::Serialize;
use std::iter;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

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

pub struct GitRepository {
    pub git_username: String,
    pub git_access_token: String,
}

#[async_trait(?Send)]
impl GitRepositoryAbstract for GitRepository {
    async fn get_committed_repos(
        &self,
        date: &DateTime,
    ) -> Result<Vec<ContributedRepository>, ApplicationError> {
        let variables = CommittedRepoVariables {
            login: self.git_username.to_string(),
            date: date.to_utc_date(),
        };
        let request_body = SingleDayCommittedRepo::build_query(variables);
        let parsed_response = send_github_request(&request_body)
            .await
            .unwrap()
            .json::<Response<CommittedRepoResponse>>()
            .await
            .map_err(|_error| ApplicationError::JsonDeserializeError)?;

        let commit_contributions = parsed_response
            .data
            .unwrap()
            .user
            .unwrap()
            .contributions_collection
            .commit_contributions_by_repository;

        Ok(commit_contributions)
    }

    async fn get_commit_count(&self, date: &DateTime) -> Result<u32, ApplicationError> {
        let variables = CommitCountVariables {
            login: GIT_USERNAME.to_string(),
            date: date.to_utc_date(),
        };

        let request_body = SingleDayContributions::build_query(variables);

        let parsed_response = send_github_request(&request_body)
            .await
            .unwrap()
            .json::<Response<CommitCountResponse>>()
            .await
            .map_err(|_error| ApplicationError::JsonDeserializeError)?;

        // TODO: Error handling
        let contributions_data = parsed_response.data.unwrap();
        let user_contributions = contributions_data.user.unwrap().contributions_collection;
        let contributions_count = user_contributions.contribution_calendar.weeks[0]
            .contribution_days[0]
            .contribution_count as u32;

        Ok(contributions_count)
    }
}
