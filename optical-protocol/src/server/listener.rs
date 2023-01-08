use std::{
    io::Cursor,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::format::types::read_var_int;
use anyhow::Result;
use async_recursion::async_recursion;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    runtime::Runtime,
    task::JoinHandle,
};
use unwrap_or::unwrap_some_or;

pub enum ProtocolState {
    /// Before the status and login states
    Void,
    Status,
    Login,
    Play,
}

pub struct Connection {
    pub protocol_state: ProtocolState,
    pub packets: Receiver<Cursor<Vec<u8>>>,
}

pub fn start(rt: &mut Runtime) -> Result<Receiver<Connection>> {
    let (connections_sender, connections_receiver): (Sender<Connection>, Receiver<Connection>) =
        mpsc::channel();

    println!("time to listen???");

    let _: JoinHandle<Result<()>> = rt.spawn(async move {
        println!("time to listen!");
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        loop {
            // Accept a connection
            let (socket, _) = match listener.accept().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            let (packet_sender, packet_receiver): (
                Sender<Cursor<Vec<u8>>>,
                Receiver<Cursor<Vec<u8>>>,
            ) = mpsc::channel();

            match connections_sender.send(Connection {
                protocol_state: ProtocolState::Void,
                packets: packet_receiver,
            }) {
                Err(e) => {
                    error!(target: "net", "Failed connecting a client to the world: {}", e);
                    continue;
                }
                _ => {}
            };

            let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                // Create a buffered socket
                let mut socket = new_buffered_socket(socket);

                loop {
                    // Read a packet
                    let packet = unwrap_some_or!(read_packet(&mut socket).await?, return Ok(()));

                    // Send it across the channel
                    packet_sender.send(packet)?;
                }
            });

            // Error handling thread
            let _: JoinHandle<Result<()>> = tokio::spawn(async move {
                match handle.await? {
                    Ok(_) => info!(target: "net", "A connection stopped."),
                    Err(e) => error!(target: "net", "A connection stopped with error: {}", e),
                };

                return Ok(());
            });
        }
    });

    return Ok(connections_receiver);
}

struct BufferedSocket {
    buf: Vec<u8>,
    socket: TcpStream,
}

fn new_buffered_socket(socket: TcpStream) -> BufferedSocket {
    return BufferedSocket {
        buf: vec![],
        socket: socket,
    };
}

/// Reads some bytes from the socket's tcp socket and
/// populates the buffer.
async fn populate_socket(socket: &mut BufferedSocket) -> Result<Option<()>> {
    let n = socket.socket.read_buf(&mut socket.buf).await?;
    if n == 0 {
        return Ok(None);
    }
    return Ok(Some(()));
}

/// Returns a complete packet from a socket. Returns None if
/// the connection closed and the socket can no longer provide
/// packets.
#[async_recursion]
async fn read_packet(socket: &mut BufferedSocket) -> Result<Option<Cursor<Vec<u8>>>> {
    // Attempt reading a packet length
    let mut reader = Cursor::new(std::mem::take(&mut socket.buf));
    let length = match read_var_int(&mut reader) {
        Ok(n) => n,
        Err(_) => {
            // Not enough data, populate
            match populate_socket(socket).await? {
                None => return Ok(None),
                _ => {}
            };
            return read_packet(socket).await;
        }
    };
    let length_data = length.value;
    let length_tag = length.size;

    // Calculations after this need packet as a vec, not a cursor
    socket.buf = reader.into_inner();

    // Check if the buffer has enough to pop packet
    let length_data: usize = length_data.try_into()?;
    let length_entire_packet = length_data + length_tag;
    if length_entire_packet > socket.buf.len() || length_data == 0 {
        // Entire packet isn't buffered yet, populate
        match populate_socket(socket).await? {
            None => return Ok(None),
            _ => {}
        };
        return read_packet(socket).await;
    } else {
        // An entire packet is available

        // Split the buffer
        let remaining_buf = socket.buf.split_off(length_entire_packet);
        // Get the packet
        let mut packet = std::mem::replace(&mut socket.buf, remaining_buf);
        // Truncate it so the length is accurate
        packet.truncate(length_entire_packet);

        return Ok(Some(Cursor::new(packet)));
    }
}
