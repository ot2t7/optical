//! Common types used in the Minecraft protocol format.

use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

use anyhow::Result;
use wasabi_leb128::{ReadLeb128, WriteLeb128};

#[derive(Default, Debug, Clone)]
pub struct VarInt {
    pub value: i32,
    pub size: usize,
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        return VarInt {
            value,
            // TODO: Make this accurate, but quickly generatyed
            size: wasabi_leb128::max_bytes::<i32>(),
        };
    }
}

pub fn read_var_int(buf: &mut Cursor<Vec<u8>>) -> Result<VarInt> {
    let res = buf.read_leb128()?;
    return Ok(VarInt {
        value: res.0,
        size: res.1,
    });
}

pub fn write_var_int(buf: &mut Vec<u8>, value: i32) -> Result<usize> {
    return Ok(buf.write_leb128(value)?);
}

#[derive(Default, Debug, Clone)]
pub struct VarLong {
    pub value: i64,
    pub size: usize,
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        return VarLong {
            value,
            // TODO: Make this accurate, but quickly generatyed
            size: wasabi_leb128::max_bytes::<i64>(),
        };
    }
}

pub fn read_var_long(buf: &mut Cursor<Vec<u8>>) -> Result<VarLong> {
    let res = buf.read_leb128()?;
    return Ok(VarLong {
        value: res.0,
        size: res.1,
    });
}

pub fn write_var_long(buf: &mut Vec<u8>, value: i64) -> Result<usize> {
    return Ok(buf.write_leb128(value)?);
}

pub fn read_string(buf: &mut Cursor<Vec<u8>>) -> Result<String> {
    let len = read_var_int(buf)?;
    let mut res = String::with_capacity(len.value.try_into()?);

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
