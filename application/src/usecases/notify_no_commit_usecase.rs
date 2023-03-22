use crate::domains::enums::application_error::ApplicationError;

use super::abstract_usecase::AbstractUsecase;
use async_trait::async_trait;

pub struct NotifyNoCommitUsecase {}

#[async_trait(?Send)]
impl AbstractUsecase<String> for NotifyNoCommitUsecase {
    async fn execute(&self) -> Result<String, ApplicationError> {
        todo!()
    }
}
