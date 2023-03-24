use super::{
    constants::endpoint::GITHUB_ENDPOINT,
    queries::{single_day_comitted_repos, single_day_contributions},
    request_utils::run_graphql_query::run_graphql_query,
};
use application::{
    domains::{
        entities::contributed_repository::ContributedRepository,
        enums::application_error::ApplicationError,
        value_objects::date_time::{self, DateTime},
    },
    repositories::git_repository_abstract::GitRepositoryAbstract,
};
use async_trait::async_trait;
use std::str::FromStr;

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
            .ok_or(ApplicationError::DeserializeError)?
            .user
            .ok_or(ApplicationError::DeserializeError)?
            .contributions_collection
            .commit_contributions_by_repository
            .iter()
            .filter_map(|contribution| {
                let nodes_option = contribution.contributions.nodes;

                if nodes_option.is_none() {
                    return None;
                }

                let contributed_repositories = nodes_option
                    .unwrap()
                    .iter()
                    .filter_map(|created_commit_contribution_option| {
                        if created_commit_contribution_option.is_none() {
                            return None;
                        }

                        let contribution = created_commit_contribution_option.unwrap();
                        let repository = &contribution.repository;
                        let date_time =
                            date_time::DateTime::from_str(&*contribution.occurred_at.0).ok()?;

                        Some(ContributedRepository::new(
                            &(contribution.commit_count as u32),
                            &date_time,
                            &repository.name_with_owner,
                            &repository.url.0,
                            &repository.open_graph_image_url.0,
                        ))
                    })
                    .collect::<Vec<ContributedRepository>>();

                Some(contributed_repositories)
            })
            .flatten()
            .collect::<Vec<ContributedRepository>>();

        Ok(contributed_repositories)
    }

    async fn get_commit_count(&self, date: &DateTime) -> Result<u32, ApplicationError> {
        let request_body =
            single_day_contributions::build_query(&self.git_username, &date.to_utc_date());

        let single_day_contributions =
            run_graphql_query(&self.git_access_token, GITHUB_ENDPOINT, request_body)
                .await
                .map_err(|_error| ApplicationError::RequestError)?;

        let contributions_count = single_day_contributions
            .data
            .ok_or(ApplicationError::DeserializeError)?
            .user
            .ok_or(ApplicationError::DeserializeError)?
            .contributions_collection
            .contribution_calendar
            .weeks[0]
            .contribution_days[0]
            .contribution_count as u32;

        Ok(contributions_count)
    }
}
