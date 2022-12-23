use crate::types::{read_string, read_var_int};
use anyhow::Result;
use async_recursion::async_recursion;
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

pub struct Sentinel<'a> {
    buf: Vec<u8>,
    protocol: ProtocolState,
    socket: &'a mut TcpStream,
}

fn new_sentinel(socket: &mut TcpStream) -> Sentinel {
    return Sentinel {
        buf: vec![],
        protocol: ProtocolState::Void,
        socket: socket,
    };
}

async fn populate_sentinel(sentinel: &mut Sentinel<'_>) -> Result<Option<()>> {
    let n = sentinel.socket.read_buf(&mut sentinel.buf).await?;
    if n == 0 {
        return Ok(None);
    }
    return Ok(Some(()));
}

#[async_recursion]
async fn read_packet(sentinel: &mut Sentinel<'_>) -> Result<Option<Vec<u8>>> {
    // Attempt reading a packet length
    let reader = sentinel.buf.as_slice();
    let (length_data, length_tag) = match read_var_int(reader) {
        Ok(n) => n,
        Err(_) => {
            // Not enough data, populate
            match populate_sentinel(sentinel).await? {
                None => return Ok(None),
                _ => {}
            };
            return read_packet(sentinel).await;
        }
    };

    // Check if the buffer has enough to pop packet
    let length_data: usize = length_data.try_into()?;
    let length_entire_packet = length_data + length_tag;
    if length_entire_packet > sentinel.buf.len() || length_data == 0 {
        // Entire packet isn't buffered yet, populate
        match populate_sentinel(sentinel).await? {
            None => return Ok(None),
            _ => {}
        };
        return read_packet(sentinel).await;
    } else {
        // An entire packet is available

        // Split the buffer
        let remaining_buf = sentinel.buf.split_off(length_entire_packet);
        // Get the packet
        let mut packet = std::mem::replace(&mut sentinel.buf, remaining_buf);
        // Truncate it so the length is accurate
        packet.truncate(length_entire_packet);

        return Ok(Some(packet));
    }
}

async fn manage_connection(mut socket: &mut TcpStream) -> Result<()> {
    let mut sentinel = new_sentinel(&mut socket);

    loop {
        match read_packet(&mut sentinel).await? {
            None => return Ok(()),
            _ => {}
        }
    }
}
