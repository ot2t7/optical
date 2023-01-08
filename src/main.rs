mod net;

use std::{sync::Mutex, time};

use anyhow::Result;

use bevy_ecs::prelude::*;
use optical_protocol::{
    format::tags::{LoginPacket, PlayPacket, StatusPacket, VoidPacket},
    server,
};
use simplelog::*;
use tokio::runtime::Builder;

use crate::net::{accept_connections, packet_broadcaster, ConnectionReceiver, PacketReceived};

fn main() -> Result<()> {
    // Create the logger
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    // Create an async runtime for the async parts of the server, mostly network operations.
    let mut runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    // Create new world
    let mut world = World::new();

    // Spawn the network listener
    let connection_receiver = server::start(&mut runtime)?;
    world.insert_resource(ConnectionReceiver {
        connections: Mutex::new(connection_receiver),
    });

    // Register all events
    world.insert_resource(Events::<PacketReceived<dyn VoidPacket>>::default());
    world.insert_resource(Events::<PacketReceived<dyn StatusPacket>>::default());
    world.insert_resource(Events::<PacketReceived<dyn LoginPacket>>::default());
    world.insert_resource(Events::<PacketReceived<dyn PlayPacket>>::default());

    // Spawn the tick counter
    world.insert_resource(TickCounter::default());

    // Create a new Schedule, which defines execution strategy for systems
    let mut schedule = Schedule::default();

    // Define a unique name for the new stage
    #[derive(StageLabel)]
    pub struct UpdateLabel;

    // Add a Stage to the schedule. Add each system here
    schedule.add_stage(
        UpdateLabel,
        SystemStage::parallel()
            .with_system(update_tick_counter)
            .with_system(accept_connections)
            .with_system(packet_broadcaster)
            .with_system(Events::<PacketReceived<dyn VoidPacket>>::update_system)
            .with_system(Events::<PacketReceived<dyn StatusPacket>>::update_system)
            .with_system(Events::<PacketReceived<dyn LoginPacket>>::update_system)
            .with_system(Events::<PacketReceived<dyn PlayPacket>>::update_system),
    );

    // Run all systems
    let min_tick_duration = time::Duration::from_secs_f32(1. / 20.);
    loop {
        let before_tick = time::Instant::now();
        schedule.run(&mut world);
        let tick_duration = before_tick.elapsed();
        if tick_duration < min_tick_duration {
            let wait = min_tick_duration - tick_duration;
            std::thread::sleep(wait);
        } else {
            warn!(
                "Oh no! A tick took {} ms, more than the min of {} ms.",
                tick_duration.as_millis(),
                min_tick_duration.as_millis()
            );
        }
    }

    return Ok(());
}

#[derive(Resource, Default)]
struct TickCounter(u64);

fn update_tick_counter(mut counter: ResMut<TickCounter>) {
    counter.0 += 1;
}
