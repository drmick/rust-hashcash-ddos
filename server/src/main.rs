use std::sync::Arc;
use server::{read_file, start_server};

#[tokio::main]
async fn main() {
    let words_of_wisdom = read_file("./server/words_of_wisdom.txt");
    let words_of_wisdom = Arc::new(words_of_wisdom);
    tokio::spawn(async move {
        start_server("localhost:45000", words_of_wisdom).await;
    }).await.unwrap();
}
