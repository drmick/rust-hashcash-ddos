use std::fs::File;
use std::io::BufRead;

use sha3::{Digest, Sha3_256};
use tokio::net::{TcpListener};
use tokio::sync::mpsc::{Receiver, Sender};
use std::io::{BufReader};
use std::sync::Arc;
use lib::{hash_message_is_ok, socket_reader, socket_writer};
use rand::prelude::SliceRandom;
use tokio::io;
use tokio::sync::{mpsc, Mutex};


pub async fn start_server(address: &str, worlds_of_wisdom: Arc<Vec<String>>) {
    let used_hash: Vec<String> = vec![];
    let used_hash = Arc::new(Mutex::new(used_hash));
    println!("Server started {}", &address);
    let listener = TcpListener::bind(String::from(address)).await.unwrap();
    loop {
        let worlds_of_wisdom= Arc::clone(&worlds_of_wisdom);
        let used_hash= Arc::clone(&used_hash);
        let (socket, _) = listener.accept().await.unwrap();
        let (mut reader, mut writer) = io::split(socket);
        let (new_message_sender, mut new_message_receiver): (Sender<String>, Receiver<String>) = mpsc::channel(100);
        let (post_message_sender, mut post_message_receiver): (Sender<String>, Receiver<String>) = mpsc::channel(100);

        tokio::spawn(async move {
            let res = socket_reader(&mut reader, new_message_sender).await;
            if res.is_err() {
                println!("{:?}", res);
            }
        });

        tokio::spawn(async move {
            router(used_hash, worlds_of_wisdom, post_message_sender, &mut new_message_receiver).await;
        });

        tokio::spawn(async move {
            socket_writer(&mut writer, &mut post_message_receiver).await;
        });
    }
}

pub fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("file of words not found");
    let reader = BufReader::new(file);

    let mut vec = vec![];
    for line in reader.lines() {
        if let Ok(..) = line {
            vec.push(line.unwrap());
        }
    }
    vec
}

pub async fn router(used_hash: Arc<Mutex<Vec<String>>>,
                words_of_wisdom: Arc<Vec<String>>,
                writer: Sender<String>,
                receiver: &mut Receiver<String>) {
    let mut hasher = Sha3_256::new();
    while let Some(message) = receiver.recv().await {
        println!("Received message: {}", message);
        let (result, hash) = hash_message_is_ok(&mut hasher, &message);
        if result {
            let hash = hash.unwrap();
            println!("Message hash verified {}", hash);
            {
                let mut used_hash = used_hash.lock().await;
                if used_hash.contains(&hash) {
                    // it is forbidden to use repeated identical messages
                    println!("Hash {} already used", &hash);
                    return;
                } else {
                    used_hash.push(hash);
                }
            }
            let message = words_of_wisdom.choose(&mut rand::thread_rng());
            writer.send(message.unwrap().parse().unwrap()).await.unwrap();
        } else {
            println!("Incorrect message hash")
        }
    }
}