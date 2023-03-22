use async_trait::async_trait;

use crate::domains::enums::application_error::ApplicationError;

#[async_trait(?Send)]
pub trait AbstractUsecase<T> {
    async fn execute(&self) -> Result<T, ApplicationError>;
}
