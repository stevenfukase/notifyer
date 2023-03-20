use crate::domains::application_error::ApplicationError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AbstractUsecase<T> {
    async fn execute() -> Result<T, ApplicationError>;
}
