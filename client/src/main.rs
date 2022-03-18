use client::start_client;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        start_client("localhost".parse().unwrap(), "45000".parse().unwrap()).await;
    })
    .await
    .unwrap();
}
