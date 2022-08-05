pub mod controllers;
pub mod repositories;
pub mod services;
pub mod usecases;

pub async fn run() {
    controllers::notify::notify().await;
}
