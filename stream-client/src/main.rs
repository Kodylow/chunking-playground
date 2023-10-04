use futures_util::stream::StreamExt;
use reqwest;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut response = client
        .get("http://127.0.0.1:3000")
        .send()
        .await
        .expect("Failed to send request")
        .bytes_stream();

    while let Some(item) = response.next().await {
        println!("{}", String::from_utf8_lossy(&item.unwrap()));
    }
}
