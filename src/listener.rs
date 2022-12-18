use crate::types::{read_string, read_var_int};
use anyhow::Result;
use std::io::Cursor;
use tokio::{io::AsyncReadExt, net::TcpListener};

pub async fn start() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    loop {
        // Accept a connection
        let (mut socket, _) = listener.accept().await?;

        // Spawn a thread to handle the connection
        tokio::spawn(async move {
            // The per-packet buffer
            let mut buf = vec![0; 1024];

            loop {
                // Read the data
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                // The connection closed
                if n == 0 {
                    println!("The connection has closed!");
                    //std::fs::write("out", acc).unwrap();
                    return;
                }

                // Read packet information
                let mut reader = Cursor::new(buf.clone());
                let length = read_var_int(&mut reader).await.unwrap();
                let packet_id = read_var_int(&mut reader).await.unwrap();

                println!(
                    "Recieved packet id: {} (packet length: {})",
                    packet_id, length
                );

                println!(
                    "Protocol version: {}",
                    read_var_int(&mut reader).await.unwrap()
                );

                println!("{}", read_string(&mut reader).await.unwrap());

                // Reset the buffer
                buf = vec![0; 1024];
            }
        });
    }
}
