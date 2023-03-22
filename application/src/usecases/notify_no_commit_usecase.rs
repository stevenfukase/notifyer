use super::abstract_usecase::AbstractUsecase;
use crate::{
    domains::{enums::application_error::ApplicationError, value_objects::date_time::DateTime},
    repositories::git_repository_abstract::GitRepositoryAbstract,
};
use async_trait::async_trait;

pub struct NotifyNoCommitUsecase<'a> {
    pub git_repository: &'a dyn GitRepositoryAbstract,
}

impl<'a> NotifyNoCommitUsecase<'a> {
    pub fn new(git_repository: &'a dyn GitRepositoryAbstract) -> Self {
        Self { git_repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<bool> for NotifyNoCommitUsecase<'a> {
    async fn execute(&self) -> Result<bool, ApplicationError> {
        let now = DateTime::now();
        let contribution_count = self.git_repository.get_commit_count(&now).await?;
        if contribution_count == 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
