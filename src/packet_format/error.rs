use std::fmt::Display;

use serde::{de, ser};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("'any' types are unparsable")]
    ParsingAny,
    #[error("failed parsing a varint")]
    MalformedVarInt,
    #[error("failed parsing a string")]
    MalformedString,
    #[error("failed parsing an unsigned short")]
    MalformedU16,
    #[error("failed reading a byte")]
    NoMoreBytes,
    #[error("failed reading a 32 bit signed integer")]
    MalformedI32,
    #[error("failed parsing a boolean")]
    MalformedBool,
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
