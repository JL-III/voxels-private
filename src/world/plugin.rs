use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_atmosphere::prelude::*;

use super::{
    atmosphere::{daylight_cycle, setup_environment, CycleTimer},
    block::VertexScale,
    chunk::{
        chunk_enter_listener, load_chunk, load_chunk_from_queue, player_move_event_listener,
        render, setup_initial_chunks, ChunkQueue, ChunkRadius, ChunkRegistry,
    },
    commands::{chunk_despawn_command, chunk_radius_command, spawn_cube_command},
    events::{ChunkCreatedEvent, ChunkEnterEvent, PrepareChunkLoadEvent},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRegistry {
            chunks: Vec::<Vec3>::new(),
        })
        .insert_resource(VertexScale { scale: 1.0 })
        .insert_resource(ChunkRadius { radius: 5 })
        .insert_resource(ChunkQueue { chunks: Vec::new() })
        .insert_resource(Msaa::Sample4)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(CycleTimer(Timer::new(
            Duration::from_millis(50),
            TimerMode::Repeating,
        )))
        .add_plugins((AtmospherePlugin,))
        .add_event::<ChunkCreatedEvent>()
        .add_event::<ChunkEnterEvent>()
        .add_event::<PrepareChunkLoadEvent>()
        .add_systems(Update, load_chunk)
        .add_systems(Update, load_chunk_from_queue)
        .add_systems(Update, spawn_cube_command)
        .add_systems(Update, chunk_enter_listener)
        .add_systems(Update, render)
        .add_systems(Update, player_move_event_listener)
        .add_systems(Startup, setup_environment)
        .add_systems(Update, setup_initial_chunks)
        .add_systems(Update, daylight_cycle)
        .add_systems(Update, chunk_despawn_command)
        .add_systems(Update, chunk_radius_command);
    }
}
