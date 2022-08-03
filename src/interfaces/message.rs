use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait Message {
    async fn send(message: String) -> Result<(), Box<dyn Error>>;
}
