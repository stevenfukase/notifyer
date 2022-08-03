use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait MessageRepositoryAbstract {
    async fn send(&self, message: String) -> Result<(), Box<dyn Error>>;
}
