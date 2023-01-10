//! Fast and easy library to interface with the Minecraft network protocol.
//!
//! This library provides a [`Serde`] data format to Serialize and Deserialize Minecraft packets.
//! It also defines every single vanilla Minecraft packet, as well as providing some network utils
//! like a TCP listener. One might choose to use this library to write their own Minecraft server/client
//! implementations, or to define their own custom packets for mods.
//!
//! [`Serde`]: https://docs.rs/serde/latest/serde/

#[macro_use]
extern crate log;

pub mod format;
pub mod packets;
pub mod server;
