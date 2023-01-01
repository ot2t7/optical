use std::io::Cursor;

use crate::{
    packet_defs::{Handshake, LoginStart, PingRequest, StatusRequest},
    packet_format::{
        deserializer,
        tags::VoidPacket,
        types::{read_var_int, write_string, write_var_int},
    },
};
use anyhow::Result;
use async_recursion::async_recursion;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use unwrap_or::unwrap_some_or;

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

/// Reads some bytes from the sentinel's tcp socket and
/// populates the buffer.
async fn populate_sentinel(sentinel: &mut Sentinel<'_>) -> Result<Option<()>> {
    let n = sentinel.socket.read_buf(&mut sentinel.buf).await?;
    if n == 0 {
        return Ok(None);
    }
    return Ok(Some(()));
}

/// Returns a complete packet from a sentinel. Returns None if
/// the connection closed and the sentinel can no longer provide
/// packets.
#[async_recursion]
async fn read_packet(sentinel: &mut Sentinel<'_>) -> Result<Option<Cursor<Vec<u8>>>> {
    // Attempt reading a packet length
    let mut reader = Cursor::new(std::mem::take(&mut sentinel.buf));
    let length = match read_var_int(&mut reader) {
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
    let length_data = length.value;
    let length_tag = length.size;

    // Calculations after this need packet as a vec, not a cursor
    sentinel.buf = reader.into_inner();

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

        return Ok(Some(Cursor::new(packet)));
    }
}

async fn manage_connection(mut socket: &mut TcpStream) -> Result<()> {
    let mut sentinel = new_sentinel(&mut socket);

    // Recieve a handshake packet
    let reader = &mut unwrap_some_or!(read_packet(&mut sentinel).await?, return Ok(()));
    let handshake: Handshake = deserializer::from_bytes(reader)?;
    println!("{:#?}", handshake);

    if handshake.next_state.value == 1 {
        // Recieve a status packet
        let reader = &mut unwrap_some_or!(read_packet(&mut sentinel).await?, return Ok(()));
        let status: StatusRequest = deserializer::from_bytes(reader)?;
        println!("{:#?}", status);

        // Recieve a ping request
        let reader = &mut unwrap_some_or!(read_packet(&mut sentinel).await?, return Ok(()));
        let ping: PingRequest = deserializer::from_bytes(reader)?;
        println!("{:#?}", ping);
    } else {
        // Recieve a login start packet
        let reader = &mut unwrap_some_or!(read_packet(&mut sentinel).await?, return Ok(()));
        let login: LoginStart = deserializer::from_bytes(reader)?;
        println!("{:#?}", login);
        // Disconnect the user
        let mut buf: Vec<u8> = Vec::new();
        write_var_int(&mut buf, 0x00)?;
        write_string(
            &mut buf,
            String::from(format!(
                r#"{{
        "text": "You a whole bingle bop, {}!",
        "bold": true,
        "color": "green"
    }}"#,
                login.player_username
            )),
        )?;
        let mut length = Vec::new();
        write_var_int(&mut length, buf.len().try_into().unwrap())?;
        buf = [length, buf].concat();
        socket.write_all(&buf).await?;
    }

    return Ok(());
}
