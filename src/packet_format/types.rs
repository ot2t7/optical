use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

use anyhow::Result;
use wasabi_leb128::{ReadLeb128, WriteLeb128};

#[derive(Default, Debug)]
pub struct VarInt {
    pub value: i32,
    pub size: usize,
}

pub fn read_var_int(buf: &mut Cursor<Vec<u8>>) -> Result<VarInt> {
    let res = buf.read_leb128()?;
    return Ok(VarInt {
        value: res.0,
        size: res.1,
    });
}

pub fn write_var_int(buf: &mut Vec<u8>, value: i32) -> Result<()> {
    buf.write_leb128(value)?;
    return Ok(());
}

#[derive(Default, Debug)]
pub struct VarLong {
    pub value: i64,
    pub size: usize,
}

pub fn read_var_long(buf: &mut Cursor<Vec<u8>>) -> Result<VarLong> {
    let res = buf.read_leb128()?;
    return Ok(VarLong {
        value: res.0,
        size: res.1,
    });
}

pub fn write_var_long(buf: &mut Vec<u8>, value: i64) -> Result<()> {
    buf.write_leb128(value)?;
    return Ok(());
}

pub fn read_string(buf: &mut Cursor<Vec<u8>>) -> Result<String> {
    let len = read_var_int(buf)?;
    let mut res = String::with_capacity(len.value as usize);

    for _ in 0..len.value {
        let mut byte = [0u8];
        buf.read_exact(&mut byte)?;
        res.push(byte[0].into());
    }

    return Ok(res);
}

pub fn write_string(buf: &mut Vec<u8>, string_to_pack: impl Into<String>) -> Result<()> {
    let string: String = string_to_pack.into();
    write_var_int(buf, string.len().try_into()?)?;
    buf.append(&mut string.into_bytes());

    return Ok(());
}

#[derive(Debug)]
pub struct MinecraftUuid(pub uuid::Uuid);
