use std::io::Cursor;

use anyhow::Result;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use wasabi_leb128::{ReadLeb128, WriteLeb128};

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("attempted parsing a varint with {0} bytes, the max bytes are 5")]
    VarIntTooBig(usize),
}

pub fn read_var_int(buff: &mut Cursor<Vec<u8>>) -> Result<i32> {
    let (value, bytes_read): (i32, usize) = buff.read_leb128()?;
    if bytes_read > 5 {
        return Err(ParsingError::VarIntTooBig(bytes_read).into());
    }
    return Ok(value);
}

pub fn write_var_int(buff: &mut Vec<u8>, value: i32) -> Result<()> {
    buff.write_leb128(value)?;
    return Ok(());
}

pub async fn read_string(buff: &mut Cursor<Vec<u8>>) -> Result<String> {
    let len = read_var_int(buff)?;
    let mut res = String::with_capacity(len as usize);

    for _ in 0..len {
        res.push(buff.read_u8().await?.into());
    }

    return Ok(res);
}

pub async fn write_string(buff: &mut Vec<u8>, string: &str) -> Result<()> {
    write_var_int(buff, string.len().try_into()?)?;
    buff.append(&mut string.to_string().into_bytes());

    return Ok(());
}
