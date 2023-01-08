use bevy_ecs::prelude::*;
use log::{error, info};
use optical_protocol::{
    format::{
        deserializer,
        tags::{LoginPacket, PlayPacket, StatusPacket, VoidPacket},
    },
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
            protocol_state: value.protocol_state,
            packets: Mutex::new(value.packets),
        };
    }
}

pub struct PacketReceived<T: ?Sized> {
    pub target: u32,
    pub content: Box<T>,
}

macro_rules! recv_packet {
    ($w:ident, $packet:ident, $entity:ident) => {{
        let deserialized_packet = match deserializer::from_bytes_generic(&mut $packet) {
            Ok(n) => n,
            Err(e) => {
                error!("Failed deserializing a client's packet: {}", e);
                continue;
            }
        };
        $w.send(PacketReceived {
            target: $entity.index(),
            content: deserialized_packet,
        })
    }};
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
        info!("Found a network connected entity: {}", entity.index());
        loop {
            match conn.packets.lock().unwrap().try_recv() {
                Ok(mut packet) => {
                    info!("Found a packet from network connected entity!");
                    match conn.protocol_state {
                        ProtocolState::Void => recv_packet!(void_writer, packet, entity),
                        ProtocolState::Status => recv_packet!(status_writer, packet, entity),
                        ProtocolState::Login => recv_packet!(login_writer, packet, entity),
                        ProtocolState::Play => recv_packet!(play_writer, packet, entity),
                    }
                }
                Err(TryRecvError::Disconnected) => commands.entity(entity).despawn(),
                Err(_) => break,
            }
        }
    }
}
