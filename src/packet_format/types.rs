use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

use anyhow::Result;
use serde::{de::Visitor, Deserialize, Serialize};
use wasabi_leb128::{ReadLeb128, WriteLeb128};

use serde::de::Error as SerdeError;

use super::error::Error;

#[derive(Default, Debug)]
pub struct VarInt {
    pub value: i32,
    pub size: usize,
}

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let res = deserializer.deserialize_seq(VarIntVisitor)?;
        return Ok(res.map_err(|e| SerdeError::custom(e))?);
    }
}

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

struct VarIntVisitor;

impl<'de> Visitor<'de> for VarIntVisitor {
    type Value = Result<VarInt, Error>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a varint")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        pub const CONTINUE_BIT: u8 = 0b10000000;
        let mut buf = [0u8; 5];
        let mut filled = 0;
        loop {
            let next_byte: u8 = match seq.next_element()? {
                Some(n) => n,
                None => return Ok(Err(Error::MalformedVarInt)),
            };
            buf[filled] = next_byte;
            filled += 1;
            if next_byte & CONTINUE_BIT == 0 {
                break;
            }
        }
        return Ok(read_var_int(&mut Cursor::new(buf.to_vec())).map_err(|_| Error::MalformedVarInt));
    }
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
