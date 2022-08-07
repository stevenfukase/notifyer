pub mod modules;
pub mod services;
use crate::modules::notify::notify;

pub async fn run() {
    notify().await;
}
