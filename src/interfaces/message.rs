use async_trait::async_trait;

#[async_trait]
pub trait Message {
    async fn send() -> Result<(), _>;
}
