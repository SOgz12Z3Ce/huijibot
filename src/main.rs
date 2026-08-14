use huijibot;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let token = huijibot::get_csrf_token(&client).await;
    println!("{token}");
}
