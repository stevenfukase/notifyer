#[tokio::main]
async fn main() {
    println!("Started main");
    let response = reqwest::get("https://api.github.com/users/stevenfukase").await;
    println!("{:?}", response);
}
