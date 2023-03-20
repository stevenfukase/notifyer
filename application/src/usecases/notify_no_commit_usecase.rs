use super::abstract_usecase::AbstractUsecase;
use crate::domains::application_error::ApplicationError;
use async_trait::async_trait;

pub struct NotifyNoCommitUsecase {}

#[async_trait(?Send)]
impl AbstractUsecase<String> for NotifyNoCommitUsecase {
    async fn execute() -> Result<String, ApplicationError> {
        todo!()
    }
}
