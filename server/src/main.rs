use std::sync::Arc;
use server::{read_file, start_server};

#[tokio::main]
async fn main() {
    let worlds_of_wisdom = read_file("./server/words_of_wisdom.txt");
    let worlds_of_wisdom = Arc::new(worlds_of_wisdom);
    tokio::spawn(async move {
        start_server( "localhost:45000", worlds_of_wisdom).await;
    }).await.unwrap();
}




