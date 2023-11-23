use bevy::prelude::*;

use super::chunk::Chunk;

#[derive(Event)]
pub struct ChunkCreatedEvent {
    pub chunk: Chunk,
}

#[derive(Event)]
pub struct ChunkEnterEvent {
    pub chunk_x: isize,
    pub chunk_z: isize,
}