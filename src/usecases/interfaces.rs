
#[async_trait(?Send)]
pub trait AbstractUsecase<T> {
    async fn execute(&self) -> Result<T, Error>;
}
