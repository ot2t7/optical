use std::io::Cursor;

use tokio::{io, io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};

/// This function fails when an invalid `VarInt` is read
/// (more than 32 bits).
async fn read_var_int(packet: &mut Cursor<Vec<u8>>) -> io::Result<i32> {
    const SEGMENT_BITS: u8 = 0x7f;
    const CONTINUE_BIT: u8 = 0x80;

    let mut value: i32 = 0;
    let mut position: u8 = 0;

    loop {
        let current_byte = packet.read_u8().await?;
        value |= ((current_byte & SEGMENT_BITS) as i32) << position;

        if current_byte & CONTINUE_BIT == 0 {
            break;
        };

        position += 7;

        if position >= 32 {
            return Err(io::ErrorKind::InvalidInput.into());
        }
    }

    return Ok(value);
}

async fn read_string(packet: &mut Cursor<Vec<u8>>) -> io::Result<String> {
    let len = read_var_int(packet).await?;
    let mut res = String::with_capacity(len as usize);

    for _ in 0..len {
        res.push(packet.read_u8().await?.into());
    }

    return Ok(res);
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // The accumulator
            let mut acc = vec![];
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

                // Append to the accumulator
                acc.append(&mut buf.drain(0..n).collect());

                // Reset the buffer
                buf = vec![0; 1024];
            }
        });
    }
}
