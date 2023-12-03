use bevy::{
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        system::Resource,
    },
    math::Vec3,
};
use serde::{Deserialize, Serialize};

use crate::{player::client::events::PlayerMoveEvent, world::events::ChunkEnterEvent};

use super::block::Block;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_VERT: i32 = 16;

#[derive(Serialize, Deserialize, Component, Clone, Copy, PartialEq, Debug)]
pub struct Chunk {
    pub chunk_x: f32,
    pub chunk_z: f32,
    pub chunk_y: f32,
    pub blocks: [[[Block; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
}

pub fn player_move_event_listener(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut enter_chunk_event_writer: EventWriter<ChunkEnterEvent>,
) {
    for event in player_move_event_reader.read() {
        let starting_chunk_x = (event.starting_position.x / 16.0).floor() as i32;
        let starting_chunk_y = (event.starting_position.y / 16.0).floor() as i32;
        let starting_chunk_z = (event.starting_position.z / 16.0).floor() as i32;

        let final_chunk_x = (event.final_position.x / 16.0).floor() as i32;
        let final_chunk_y = (event.final_position.y / 16.0).floor() as i32;
        let final_chunk_z = (event.final_position.z / 16.0).floor() as i32;

        if starting_chunk_x != final_chunk_x
            || starting_chunk_y != final_chunk_y
            || starting_chunk_z != final_chunk_z
        {
            enter_chunk_event_writer.send(ChunkEnterEvent {
                chunk_coords: Vec3::new(
                    final_chunk_x as f32,
                    final_chunk_y as f32,
                    final_chunk_z as f32,
                ),
            });
        }
    }
}

#[derive(Resource)]
pub struct ChunkRadius {
    pub radius: i32,
}

#[derive(Resource)]
pub struct ChunkQueue {
    pub chunks: Vec<Vec3>,
}

#[derive(Resource)]
pub struct ChunkRegistry {
    pub chunks: Vec<Vec3>,
}
