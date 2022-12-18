use std::io::Cursor;

use anyhow::Result;
use thiserror::Error;
use tokio::io::AsyncReadExt;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("attempted parsing a varint with {0} bits, the max bits are 32")]
    VarIntTooBig(u32),
}

pub async fn read_var_int(packet: &mut Cursor<Vec<u8>>) -> Result<i32> {
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
            return Err(ParsingError::VarIntTooBig(position as u32))?;
        }
    }

    return Ok(value);
}

pub async fn read_string(packet: &mut Cursor<Vec<u8>>) -> Result<String> {
    let len = read_var_int(packet).await?;
    let mut res = String::with_capacity(len as usize);

    for _ in 0..len {
        res.push(packet.read_u8().await?.into());
    }

    return Ok(res);
}
