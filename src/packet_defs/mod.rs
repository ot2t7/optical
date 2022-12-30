use crate::packet_format::{
    tags::*,
    types::{MinecraftUuid, VarInt},
};
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
    pub rand_number: i64,
}
#[typetag::serde(name = "1")]
impl StatusPacket for PingRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStart {
    pub player_username: String,
    pub uuid: Option<MinecraftUuid>,
}
#[typetag::serde(name = "0")]
impl LoginPacket for PingRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
#[typetag::serde(name = "1")]
impl LoginPacket for EncryptionResponse {}
