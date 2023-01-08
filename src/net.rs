use std::{
    io::Cursor,
    sync::{mpsc::Receiver, Mutex},
};

use bevy_ecs::prelude::*;
use optical_protocol::{
    format::{
        deserializer,
        tags::{LoginPacket, PlayPacket, StatusPacket, VoidPacket},
    },
    server::{Connection, ProtocolState},
};

use log::{error, info};

#[derive(Component)]
pub struct NetworkConnected {
    protocol_state: ProtocolState,
    /// This component will not be accessed in parallel due to how ECS works. The Mutex is only here
    /// to make `Reciever` sendable through threads.
    packets: Mutex<Receiver<Cursor<Vec<u8>>>>,
}

impl From<Connection> for NetworkConnected {
    fn from(value: Connection) -> Self {
        return NetworkConnected {
            protocol_state: value.protocol_state,
            packets: Mutex::new(value.packets),
        };
    }
}

pub struct PacketReceived<T> {
    target: u32,
    content: T,
}

pub fn packet_broadcaster(
    mut query: Query<(Entity, &mut NetworkConnected)>,
    mut void_writer: EventWriter<PacketReceived<Box<dyn VoidPacket>>>,
    mut status_writer: EventWriter<PacketReceived<Box<dyn StatusPacket>>>,
    mut login_writer: EventWriter<PacketReceived<Box<dyn VoidPacket>>>,
    mut play_writer: EventWriter<PacketReceived<Box<dyn VoidPacket>>>,
) {
    for (entity, conn) in &mut query {
        while let Ok(mut packet) = conn.packets.lock().unwrap().try_recv() {
            match conn.protocol_state {
                ProtocolState::Void => {
                    let deserialized_packet = match deserializer::from_bytes_generic(&mut packet) {
                        Ok(n) => n,
                        Err(e) => {
                            error!("Failed deserializing a client's packet: {}", e);
                            continue;
                        }
                    };
                    void_writer.send(PacketReceived {
                        target: entity.index(),
                        content: deserialized_packet,
                    })
                }
                ProtocolState::Status => {
                    let deserialized_packet = match deserializer::from_bytes_generic(&mut packet) {
                        Ok(n) => n,
                        Err(e) => {
                            error!("Failed deserializing a client's packet: {}", e);
                            continue;
                        }
                    };
                    status_writer.send(PacketReceived {
                        target: entity.index(),
                        content: deserialized_packet,
                    })
                }
                ProtocolState::Login => {
                    let deserialized_packet = match deserializer::from_bytes_generic(&mut packet) {
                        Ok(n) => n,
                        Err(e) => {
                            error!("Failed deserializing a client's packet: {}", e);
                            continue;
                        }
                    };
                    login_writer.send(PacketReceived {
                        target: entity.index(),
                        content: deserialized_packet,
                    })
                }
                ProtocolState::Play => {
                    let deserialized_packet = match deserializer::from_bytes_generic(&mut packet) {
                        Ok(n) => n,
                        Err(e) => {
                            error!("Failed deserializing a client's packet: {}", e);
                            continue;
                        }
                    };
                    play_writer.send(PacketReceived {
                        target: entity.index(),
                        content: deserialized_packet,
                    })
                }
            }
        }
    }
}
