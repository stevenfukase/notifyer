pub mod modules;

pub async fn run() {
    modules::notify::notify().await;
}
