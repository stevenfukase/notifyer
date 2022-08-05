use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AbstractUsecase<T> {
    async fn execute(&self) -> Result<T, std::io::Error>;
}
