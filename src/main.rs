mod net;

use std::time;

use anyhow::Result;

use bevy_ecs::prelude::*;
use optical_protocol::server;
use simplelog::*;
use tokio::runtime::Builder;

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
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    // Spawn the network listener,
    let connection_receiver = server::start()?;

    // Create new world
    let mut world = World::new();

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
        SystemStage::parallel().with_system(update_tick_counter),
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
}

#[derive(Resource, Default)]
struct TickCounter(u64);

fn update_tick_counter(mut counter: ResMut<TickCounter>) {
    counter.0 += 1;
}
