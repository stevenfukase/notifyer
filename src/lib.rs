pub mod features;
pub mod services;

pub async fn run() {
    features::notify::notify().await;
}
