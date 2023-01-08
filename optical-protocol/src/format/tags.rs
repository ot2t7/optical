use std::fmt::Debug;

pub trait Packet {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait VoidPacket: Debug + Send + Sync {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait StatusPacket: Debug + Send + Sync {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait LoginPacket: Debug + Send + Sync {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait PlayPacket: Debug + Send + Sync {}
