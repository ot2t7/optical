use crate::types::{read_string, read_var_int};
use anyhow::Result;
use bytes::{BufMut, BytesMut};
use std::io::Cursor;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

enum ProtocolState {
    /// Before the status and login states
    Void,
    Status,
    Login,
    Play,
}

pub async fn start() -> Result<()> {
    let _: Result<()> = tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:25565").await?;

        loop {
            // Accept a connection
            let (mut socket, _) = match listener.accept().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            // Spawn a thread to handle the connection
            tokio::spawn(async move {
                let connection_closed = manage_connection(&mut socket).await;

                match connection_closed {
                    Ok(_) => println!("A connection was closed successfully!"),
                    Err(e) => {
                        eprintln!("A connection was closed with error: {:?}!", e);
                        socket.write_all(b"Goodbye Bozo\n").await.ok();
                    }
                }
            });
        }
    })
    .await?;

    return Ok(());
}

async fn manage_connection(socket: &mut TcpStream) -> Result<()> {
    // Reserve a buffer for the packets at 2 mb, will grow in size as needed
    let mut buf = vec![0u8; 1024 * 1024 * 2];
    // Create state for the connection
    let mut state = ProtocolState::Void;

    loop {
        // Read the data
        let n = socket.read_buf(&mut buf).await?;

        // The connection closed without error
        if n == 0 {
            return Ok(());
        }

        // Attempt reading some packet info
        let reader = buf.as_slice();
        let length = match read_var_int(reader) {
            Ok(n) => n,
            Err(_) => continue, // Read more data
        };

        // Can we process an entire packet?
        let length_usize: usize = length.0.try_into()?;
        let length_entire_packet = length_usize + length.1;
        if length_entire_packet > buf.len() || buf.len() == 0 {
            // An entire packet isn't buffered yet
            continue;
        } else {
            // We can process a packet
            println!(
                "Packet len: {length_usize}, packet id: {}",
                read_var_int(reader)?.0
            );

            // Split the buffer
            let remaining_buf = buf.split_off(length_entire_packet);

            // Reset the buffer
            buf = remaining_buf;
        }
    }
}
