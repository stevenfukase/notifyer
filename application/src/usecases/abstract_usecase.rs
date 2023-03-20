use async_trait::{self, async_trait};

#[async_trait(?Send)]
pub trait AbstractUsecase<T> {
    async fn execute() -> Result<T, ApplicationError>;
}
