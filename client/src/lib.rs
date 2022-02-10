use lib::{hash_message_is_ok, send_bytes, socket_reader};
use sha3::{Digest, Sha3_256};
use tokio::io;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

pub async fn start_client(host: String, port: String) {
    let address = format!("{}:{}", host, port);
    let socket = TcpStream::connect(address).await.unwrap();
    let (mut reader, mut writer) = io::split(socket);

    let (new_message_sender, mut new_message_receiver): (Sender<String>, Receiver<String>) = mpsc::channel(100);

    tokio::spawn(async move {
        let res = socket_reader(&mut reader, new_message_sender).await;
        if res.is_err() {
            println!("{:?}", res);
        }
    });
    let message = Uuid::new_v4();
    let message = solve_hash(message);
    send_bytes(&mut writer, &message).await;
    router(&mut new_message_receiver).await;
}

async fn router(receiver: &mut Receiver<String>) {
    while let Some(message) = receiver.recv().await {
        println!("Received message: {}", message);
    }
}

pub fn solve_hash(message: Uuid) -> String {
    println!("Solving hash started for message: {}", message);
    let mut hasher = Sha3_256::new();
    let mut i:i32 = 0;
    let mut result_string ;
    loop {
        i = i + 1;
        result_string = format!("{}:{}", message, i);
        if hash_message_is_ok(&mut hasher, &result_string).0 {
            println!("Solving hash completed");
            return result_string;
        }
    }
}
