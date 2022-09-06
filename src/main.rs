#[tokio::main]
async fn main() {
    env_logger::init();
    notifyer::run().await;
}
