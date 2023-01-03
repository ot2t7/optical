use std::fmt::Debug;

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait VoidPacket: Debug + Send {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait StatusPacket: Debug + Send {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait LoginPacket: Debug + Send {}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait PlayPacket: Debug + Send {}

// Packets deserialized as generic trait objects will
// be forced to look like Box<dyn Packet> by typetag.
// By restricting T on the Boxed trait, some generic
// environment can have a good idea if T is a packet
// trait object or not. This is useful because concrete
// packet type deserialization is slightly different
// from generic packet deserialization.
pub trait Boxed {}
impl<T> Boxed for Box<T> {}
