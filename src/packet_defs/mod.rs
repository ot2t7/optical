use crate::packet_format::{tags::*, types::VarInt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}
#[typetag::serde(name = "0")]
impl VoidPacket for Handshake {}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusRequest {}
#[typetag::serde(name = "0")]
impl StatusPacket for StatusRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub rand_number: [u16; 4],
}
#[typetag::serde(name = "1")]
impl StatusPacket for PingRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStart {
    pub player_username: String,
    pub has_uuid: bool,
    pub uuid: [i32; 2],
}
#[typetag::serde(name = "0")]
impl LoginPacket for PingRequest {}
