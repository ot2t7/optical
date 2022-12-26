use std::fmt::Debug;

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait VoidPacket: Debug {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait StatusPacket: Debug {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait LoginPacket: Debug {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait PlayPacket: Debug {}
