use bevy_ecs::prelude::*;
use log::error;
use optical_protocol::{
    format::{
        deserializer,
        tags::{LoginPacket, PlayPacket, StatusPacket, VoidPacket},
    },
    packets::void::serverbound::Handshake,
    server::{Connection, ProtocolState},
};
use std::{
    io::Cursor,
    sync::{
        mpsc::{Receiver, TryRecvError},
        Mutex,
    },
};

#[derive(Resource)]
pub struct ConnectionReceiver {
    pub connections: Mutex<Receiver<Connection>>,
}

pub fn accept_connections(receiver: ResMut<ConnectionReceiver>, mut commands: Commands) {
    let receiver = receiver.connections.lock().unwrap();
    while let Ok(conn) = receiver.try_recv() {
        commands.spawn::<NetworkConnected>(conn.into());
    }
}

#[derive(Component)]
pub struct NetworkConnected {
    pub protocol_state: ProtocolState,
    /// This component will not be accessed in parallel due to how ECS works. The Mutex is only here
    /// to make `Reciever` sendable through threads.
    pub packets: Mutex<Receiver<Cursor<Vec<u8>>>>,
}

impl From<Connection> for NetworkConnected {
    fn from(value: Connection) -> Self {
        return NetworkConnected {
            protocol_state: value.0,
            packets: Mutex::new(value.1),
        };
    }
}

pub struct PacketReceived<T: ?Sized> {
    pub target: Entity,
    pub content: Box<T>,
}

macro_rules! recv_packet {
    ($w:ident, $packet:ident, $entity:ident, $stop:literal) => {{
        let deserialized_packet = match deserializer::from_bytes_generic(&mut $packet) {
            Ok(n) => n,
            Err(e) => {
                error!("Failed deserializing a client's packet: {}", e);
                continue;
            }
        };
        $w.send(PacketReceived {
            target: $entity,
            content: deserialized_packet,
        });
        if $stop == true {
            break;
        }
    }};
}

pub fn system1(
    mut reader: EventReader<PacketReceived<dyn VoidPacket>>,
    mut query: Query<(Entity, &mut NetworkConnected)>,
) {
    for handshake in reader.iter() {
        match query.get_mut(handshake.target) {
            Ok((_, mut conn)) => {
                let d = handshake
                    .content
                    .as_any()
                    .downcast_ref::<Handshake>()
                    .unwrap();

                match d.next_state.value {
                    1 => conn.protocol_state = ProtocolState::Status,
                    2 => conn.protocol_state = ProtocolState::Login,
                    _ => {}
                };
            }
            _ => {}
        }
    }
}

pub fn packet_broadcaster(
    mut query: Query<(Entity, &mut NetworkConnected)>,
    mut void_writer: EventWriter<PacketReceived<dyn VoidPacket>>,
    mut status_writer: EventWriter<PacketReceived<dyn StatusPacket>>,
    mut login_writer: EventWriter<PacketReceived<dyn LoginPacket>>,
    mut play_writer: EventWriter<PacketReceived<dyn PlayPacket>>,
    mut commands: Commands,
) {
    for (entity, conn) in &mut query {
        loop {
            match conn.packets.lock().unwrap().try_recv() {
                Ok(mut packet) => match conn.protocol_state {
                    // Broadcast void/status/login packets once / tick / client, because a protocol
                    // state switch may occur after each packet, forcing the next packet to go into
                    // the wrong event queue.
                    ProtocolState::Void => recv_packet!(void_writer, packet, entity, true),
                    ProtocolState::Status => recv_packet!(status_writer, packet, entity, true),
                    ProtocolState::Login => recv_packet!(login_writer, packet, entity, true),
                    // The protocol state doesn't switch anymore in the play state. Process packets
                    // in batches.
                    ProtocolState::Play => recv_packet!(play_writer, packet, entity, false),
                },
                Err(TryRecvError::Disconnected) => {
                    commands.entity(entity).despawn();
                    break;
                }
                Err(_) => break,
            }
        }
    }
}
