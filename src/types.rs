use std::io::Cursor;

use anyhow::Result;
use bytes::{Buf, BytesMut};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use wasabi_leb128::{ReadLeb128, WriteLeb128};

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("attempted parsing a varint with {0} bytes, the max bytes are 5")]
    VarIntTooBig(usize),
}

pub fn read_var_int(mut buf: &[u8]) -> Result<(i32, usize)> {
    let (value, bytes_read): (i32, usize) = buf.read_leb128()?;
    if bytes_read > 5 {
        return Err(ParsingError::VarIntTooBig(bytes_read).into());
    }
    return Ok((value, bytes_read));
}

pub fn write_var_int(buf: &mut Vec<u8>, value: i32) -> Result<()> {
    buf.write_leb128(value)?;
    return Ok(());
}

pub async fn read_string(mut buf: &[u8]) -> Result<String> {
    let len = read_var_int(buf)?;
    let mut res = String::with_capacity(len.0 as usize);

    for _ in 0..len.0 {
        res.push(buf.read_u8().await?.into());
    }

    return Ok(res);
}

pub async fn write_string(buf: &mut Vec<u8>, string_to_pack: impl Into<String>) -> Result<()> {
    let string: String = string_to_pack.into();
    write_var_int(buf, string.len().try_into()?)?;
    buf.append(&mut string.into_bytes());

    return Ok(());
}
