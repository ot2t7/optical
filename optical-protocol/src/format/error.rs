use std::fmt::Display;

use serde::{de, ser};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    // Deserialization/serialization errors
    #[error("'any' types do not exist in this format")]
    AnyType,
    #[error("map types do not exist in this format")]
    MapType,
    #[error("char types do not exist in this format")]
    CharType,
    #[error("failed parsing a var int")]
    MalformedVarInt,
    #[error("failed parsing a var long")]
    MalformedVarLong,
    #[error("failed parsing a string")]
    MalformedString,
    #[error("failed reading a byte")]
    NoMoreBytes,
    #[error("failed parsing a 16 big unsigned integer")]
    MalformedU16,
    #[error("failed parsing a 32 big unsigned integer")]
    MalformedU32,
    #[error("failed parsing a 64 big unsigned integer")]
    MalformedU64,
    #[error("failed reading a 8 bit signed integer")]
    MalformedI8,
    #[error("failed reading a 16 bit signed integer")]
    MalformedI16,
    #[error("failed reading a 32 bit signed integer")]
    MalformedI32,
    #[error("failed reading a 64 bit signed integer")]
    MalformedI64,
    #[error("failed reading a 32 bit float")]
    MalformedF32,
    #[error("failed reading a 64 bit float")]
    MalformedF64,
    #[error("failed parsing a boolean")]
    MalformedBool,

    // Serialization errors
    #[error("attempted serializing a sequence with no known length")]
    UnsizedSeq,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
