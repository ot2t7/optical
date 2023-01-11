use std::{
    io::Cursor,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{
    format::tags::{ClientLoginPacket, LoginPacket},
    packets::login::serverbound::EncryptionResponse,
};
use crate::{
    format::{deserializer, serializer, types::read_var_int},
    packets::{
        login::{clientbound::EncryptionRequest, serverbound::LoginStart},
        void::serverbound::Handshake,
    },
};
use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use pkcs1::EncodeRsaPublicKey;
use rsa::{PaddingScheme, RsaPrivateKey, RsaPublicKey};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime::Runtime,
    sync::Mutex,
    task::JoinHandle,
};
use unwrap_or::unwrap_some_or;

/// The current state that a connection between a client and the server is in.
pub enum ProtocolState {
    /// The client and server handshake
    Void,
    /// The server provides some information about itself for the Minecraft server list
    Status,
    /// The client attempts to join the server
    Login,
    /// The client is playing on the server
    Play,
}

pub type Connection = (ProtocolState, Receiver<Cursor<Vec<u8>>>);

pub fn start(rt: &mut Runtime) -> Result<Receiver<Connection>> {
    let (connections_sender, connections_receiver): (Sender<Connection>, Receiver<Connection>) =
        mpsc::channel();

    // Generate a public key
    const bits: usize = 1024;
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    let _: JoinHandle<Result<()>> = rt.spawn(async move {
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

            // Clone the sender for this client
            let connections_sender = connections_sender.clone();

            // Clone crypto
            let public_key = public_key.clone();
            let private_key = private_key.clone();

            let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                let mut socket = new_buffered_socket(socket);

                // First, accept a handshake packet
                let handshake: Handshake = deserializer::from_bytes(&mut unwrap_some_or!(
                    read_packet(&mut socket).await?,
                    return Ok(())
                ))?;

                // Client wants the Status state
                if handshake.next_state.value == 1 {
                    // We can't respond with this packet as we don't have any information about
                    // the current world. Let's send this client over to the receiver.
                    connections_sender
                        .send((ProtocolState::Status, packet_receiver))
                        .map_err(|e| anyhow!("{e}"))?;
                    loop {
                        // Read a packet
                        let packet =
                            unwrap_some_or!(read_packet(&mut socket).await?, return Ok(()));

                        // Send it across the channel
                        packet_sender.send(packet)?;
                    }
                } else {
                    // Client wants to login into the server

                    // Process the Login Start request
                    let login_start: LoginStart = deserializer::from_bytes(&mut unwrap_some_or!(
                        read_packet(&mut socket).await?,
                        return Ok(())
                    ))?;

                    println!("Seems {} wants to login.", login_start.name);

                    let verify_token = rand::random::<[u8; 4]>().to_vec();

                    println!("Sending verify token {:?}", verify_token);

                    // Send an encryption request
                    let encryption_request = EncryptionRequest {
                        server_id: String::new(),
                        public_key: public_key.to_pkcs1_der()?.into_vec(),
                        verify_token: verify_token,
                    };
                    socket
                        .socket
                        .write_all(&mut serializer::to_bytes(
                            &encryption_request,
                            encryption_request.packet_id(),
                        )?)
                        .await?;

                    println!("Sent!");

                    // Process the Encryption Response packet
                    let encryption_response: EncryptionResponse = deserializer::from_bytes(
                        &mut unwrap_some_or!(read_packet(&mut socket).await?, return Ok(())),
                    )?;

                    println!(
                        "Got back verify token {:?}",
                        private_key.decrypt(
                            PaddingScheme::new_pkcs1v15_encrypt(),
                            &encryption_response.verify_token
                        )?
                    );
                }

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
