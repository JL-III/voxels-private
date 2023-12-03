use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_atmosphere::prelude::*;

use crate::world::block::VertexScale;
use crate::world::chunk::{ChunkQueue, ChunkRadius, ChunkRegistry};
use crate::world::events::{ChunkCreatedEvent, PrepareChunkLoadEvent, RenderChunk};

use super::atmosphere::{daylight_cycle, setup_environment, CycleTimer};
use super::client_in::get_chunk_from_server;
use super::events::render;

pub struct ClientWorldPlugin;

impl Plugin for ClientWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRegistry {
            chunks: Vec::<Vec3>::new(),
        })
        .insert_resource(VertexScale { scale: 1.0 })
        .insert_resource(ChunkRadius { radius: 3 })
        .insert_resource(ChunkQueue { chunks: Vec::new() })
        .insert_resource(Msaa::Sample4)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(CycleTimer(Timer::new(
            Duration::from_millis(50),
            TimerMode::Repeating,
        )))
        .add_plugins(AtmospherePlugin)
        .add_event::<ChunkCreatedEvent>()
        .add_event::<PrepareChunkLoadEvent>()
        .add_event::<RenderChunk>()
        .add_systems(Update, render)
        .add_systems(Startup, setup_environment)
        .add_systems(Update, daylight_cycle)
        .add_systems(Update, get_chunk_from_server);
    }
}