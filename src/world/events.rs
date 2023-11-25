use bevy::prelude::*;

use super::chunk::Chunk;

#[derive(Event)]
pub struct ChunkCreatedEvent {
    pub chunk: Chunk,
    pub chunk_id: Entity,
    pub registry_size: usize,
}

#[derive(Event)]
pub struct ChunkEnterEvent {
    pub chunk_x: isize,
    pub chunk_z: isize,
}

#[derive(Event)]
pub struct PrepareChunkLoadEvent {
    pub chunk: Chunk,
}
