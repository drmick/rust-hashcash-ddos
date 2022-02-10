use std::error::Error;
use sha3::digest::core_api::CoreWrapper;
use sha3::{Digest, Sha3_256Core};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn socket_reader(reader: &mut ReadHalf<TcpStream>,
                           new_message_sender: Sender<String>) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(&mut *reader);
    loop {
        let mut buf: [u8; 4] = [0; 4];
        // getting message length
        try_read(&mut reader, &mut buf).await?;
        let message_len = i32::from_be_bytes(buf);

        // getting message body
        let mut buf = vec![0u8; message_len.try_into().unwrap()];
        try_read(&mut reader, &mut buf).await?;
        let message = String::from_utf8(buf).ok().unwrap();
        new_message_sender.send(message).await.unwrap();
    }
}

pub async fn socket_writer(writer: &mut WriteHalf<TcpStream>, receiver: &mut Receiver<String>)  {
    while let Some(message) = receiver.recv().await {
        send_bytes(writer, &message).await;
    }
}

async fn try_read(reader: &mut BufReader<&mut ReadHalf<TcpStream>>, message_len: &mut [u8]) -> Result<(), &'static str> {
    let read_length = reader.read_exact(message_len).await;
    match read_length {
        Ok(n) if n == 0usize => {
            return Err("connection closed");
        }
        Err(_e) => {
            return Err("connection closed");
        }
        _ => {}
    };
    Ok(())
}

pub async fn send_bytes(send: &mut WriteHalf<TcpStream>, message: &str) {
    let length = i32::try_from(message.as_bytes().len()).unwrap().to_be_bytes();
    println!("Message sent: {:?}", message);
    let message = message.as_bytes();
    send.write(&length).await.unwrap();
    send.write(message).await.unwrap();
    send.flush().await.unwrap();
}

pub fn hash_message_is_ok(hasher: &mut CoreWrapper<Sha3_256Core>, string: &str) -> (bool, Option<String>) {
    let mut hash = Default::default();
    hasher.update(string);
    hasher.finalize_into_reset(&mut hash);
    let hex = format!("{:x}",hash);
    if &hex[0..5] == "00000" {
        return (true, Some(hex));
    }
    (false, None)
}
