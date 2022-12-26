use crate::packet_format::tags::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handshake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: i32,
}
#[typetag::serde(name = "0")]
impl VoidPacket for Handshake {}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusRequest {}
#[typetag::serde(name = "0")]
impl StatusPacket for StatusRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    rand_number: [u16; 4],
}
#[typetag::serde(name = "0")]
impl StatusPacket for PingRequest {}
