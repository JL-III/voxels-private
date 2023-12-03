use bevy::prelude::*;

use crate::{
    player::{client::events::PlayerMoveEvent, events::PlayerSpawnEvent},
    world::{
        block::VertexScale,
        chunk::{player_move_event_listener, ChunkQueue, ChunkRadius, ChunkRegistry},
        commands::{chunk_despawn_command, chunk_radius_command},
        events::{ChunkCreatedEvent, ChunkEnterEvent, PrepareChunkLoadEvent},
    },
};

use super::{
    chunk::{chunk_enter_listener, load_chunk, load_chunk_from_queue, setup_initial_chunks},
    server_out::send_chunk_to_client,
};

pub struct ServerWorldPlugin;

impl Plugin for ServerWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRegistry {
            chunks: Vec::<Vec3>::new(),
        })
        .insert_resource(VertexScale { scale: 1.0 })
        .insert_resource(ChunkRadius { radius: 3 })
        .insert_resource(ChunkQueue { chunks: Vec::new() })
        .add_event::<ChunkCreatedEvent>()
        .add_event::<ChunkEnterEvent>()
        .add_event::<PrepareChunkLoadEvent>()
        .add_event::<PlayerMoveEvent>()
        .add_event::<PlayerSpawnEvent>()
        .add_systems(Update, load_chunk)
        .add_systems(Update, load_chunk_from_queue)
        .add_systems(Update, chunk_enter_listener)
        .add_systems(Update, player_move_event_listener)
        .add_systems(Update, setup_initial_chunks)
        .add_systems(Update, chunk_despawn_command)
        .add_systems(Update, chunk_radius_command)
        .add_systems(Update, send_chunk_to_client);
    }
}
