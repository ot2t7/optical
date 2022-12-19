use crate::types::{read_string, read_var_int};
use anyhow::Result;
use std::io::Cursor;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn start() -> Result<()> {
    let _: Result<()> = tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:25565").await?;

        loop {
            // Accept a connection
            let (mut socket, _) = listener.accept().await?;

            // Spawn a thread to handle the connection
            tokio::spawn(async move {
                let connection_closed: Result<()> = async {
                    // Reserve a buffer for the packet at 1024 bytes, will grow in size as needed
                    let mut buf = vec![0; 1024];

                    loop {
                        // Read the data
                        let n = socket.read(&mut buf).await?;

                        // The connection closed without error
                        if n == 0 {
                            //std::fs::write("out", acc).unwrap();
                            return Ok(());
                        }

                        // Read packet information
                        let mut reader = Cursor::new(buf.clone());
                        let length = read_var_int(&mut reader)?;
                        let packet_id = read_var_int(&mut reader)?;

                        println!("Len: {length}, ID: {packet_id}");

                        if length == 1 {
                            let mut response = vec![];
                            crate::types::write_var_int(&mut response, 0)?;
                            crate::types::write_string(
                                &mut response,
                                r#"{
    "version": {
        "name": "69.420",
        "protocol": 759
    },
    "players": {
        "max": 100,
        "online": 0,
        "sample": []
    },
    "description": {
        "text": "HELLLOOOOO FROM THE RUST SERVER!!!"
    },
    "previewsChat": true,
    "enforcesSecureChat": true
}"#,
                            )
                            .await?;
                            let mut len_res = vec![];
                            crate::types::write_var_int(
                                &mut len_res,
                                response.len().try_into().unwrap(),
                            )?;
                            len_res.append(&mut response);
                            response = len_res;
                            println!("responding with {:?}", response);
                            std::fs::write("out", &response)?;
                            socket.write_all(&response).await?;
                        }

                        // Reset the buffer
                        buf = vec![0; 1024];
                    }
                }
                .await;

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
