use application::{
    domains::{
        entities::contributed_repository::ContributedRepository,
        enums::application_error::ApplicationError, value_objects::date_time::DateTime,
    },
    repositories::git_repository_abstract::GitRepositoryAbstract,
};
use async_trait::async_trait;

use reqwest::{header, Client};
use serde::Serialize;
use std::iter;

use super::{
    constants::endpoint::GITHUB_ENDPOINT, queries::single_day_comitted_repos,
    request_utils::run_graphql_query::run_graphql_query,
};

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
        let request_body =
            single_day_comitted_repos::build_query(&self.git_username, &date.to_utc_date());

        let single_day_comitted_repos =
            run_graphql_query(&self.git_access_token, GITHUB_ENDPOINT, request_body)
                .await
                .map_err(|_error| ApplicationError::RequestError)?;

        let contributed_repositories = single_day_comitted_repos
            .data
            .ok_or(ApplicationError::RequestError)?
            .user
            .ok_or(ApplicationError::RequestError)?
            .contributions_collection
            .commit_contributions_by_repository
            .iter()
            .filter_map(|contribution| {
                contribution.contributions.nodes.iter().filter_map(
                    |created_commit_contributions| {
                        created_commit_contributions_option
                            .iter()
                            .filter_map(|a| {
                                
                            })
                    },
                );
            })
            .collect::<Vec<ContributedRepository>>();

        Ok(contributed_repositories)
    }

    async fn get_commit_count(&self, date: &DateTime) -> Result<u32, ApplicationError> {
        let variables = CommitCountVariables {
            login: self.git_username.to_string(),
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
