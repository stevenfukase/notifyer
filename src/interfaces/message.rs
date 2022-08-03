pub trait Message {
    async fn send(&self, message: String) -> Result<Ok<()>, _>;
}
