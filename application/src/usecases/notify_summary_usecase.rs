use super::abstract_usecase::AbstractUsecase;
use crate::{
    domains::{
        entities::summary_entity::Summary, enums::application_error::ApplicationError,
        value_objects::date_time::DateTime,
    },
    repositories::git_repository_abstract::GitRepositoryAbstract,
};
use async_trait::async_trait;

#[non_exhaustive]
pub struct NotifySummaryUsecase<'a> {
    pub git_repository: &'a dyn GitRepositoryAbstract,
}

impl<'a> NotifySummaryUsecase<'a> {
    pub fn new(git_repository: &'a dyn GitRepositoryAbstract) -> Self {
        Self { git_repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<Summary> for NotifySummaryUsecase<'a> {
    async fn execute(&self) -> Result<Summary, ApplicationError> {
        let now = DateTime::now();
        let contributions = self.git_repository.get_committed_repos(&now).await?;
        let summary = Summary::new(&now, &contributions);
        Ok(summary)
    }
}
