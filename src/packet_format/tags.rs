use std::fmt::Debug;

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait VoidPacket: Debug + Send {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait StatusPacket: Debug + Send {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait LoginPacket: Debug + Send {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait PlayPacket: Debug + Send {}
