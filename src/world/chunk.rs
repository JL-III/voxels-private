use bevy::{
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut, Resource},
    },
    math::Vec3,
    transform::{components::Transform, TransformBundle},
};
use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};

use crate::{
    player::{client::events::PlayerMoveEvent, events::PlayerSpawnEvent},
    world::events::{ChunkCreatedEvent, ChunkEnterEvent, PrepareChunkLoadEvent},
};

use super::{block::Block, element::Element};

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

pub fn setup_initial_chunks(
    mut chunk_registry: ResMut<ChunkRegistry>,
    chunk_radius: Res<ChunkRadius>,
    mut chunk_queue: ResMut<ChunkQueue>,
    mut player_spawned_event: EventReader<PlayerSpawnEvent>,
) {
    for event in player_spawned_event.read() {
        let chunks = get_surrounding_chunks(
            convert_to_chunk_location(event.position.x),
            convert_to_chunk_location(event.position.y),
            convert_to_chunk_location(event.position.z),
            chunk_radius.radius,
        );
        for chunk in chunks.iter() {
            if !chunk_registry.chunks.contains(chunk) {
                chunk_registry.chunks.push(*chunk);
                chunk_queue.chunks.push(*chunk);
            }
        }
    }
}

pub fn chunk_enter_listener(
    mut chunk_registry: ResMut<ChunkRegistry>,
    chunk_radius: Res<ChunkRadius>,
    mut chunk_queue: ResMut<ChunkQueue>,
    mut chunk_enter_event_reader: EventReader<ChunkEnterEvent>,
) {
    for event in chunk_enter_event_reader.read() {
        println!("Server: entered new chunk!");
        let chunks = get_surrounding_chunks(
            event.chunk_coords[0] as i32,
            event.chunk_coords[1] as i32,
            event.chunk_coords[2] as i32,
            chunk_radius.radius,
        );
        for chunk in chunks.iter() {
            if !chunk_registry.chunks.contains(chunk) {
                chunk_registry.chunks.push(*chunk);
                chunk_queue.chunks.push(*chunk);
            }
        }
    }
}

pub fn load_chunk(
    mut commands: Commands,
    mut prepare_chunk_load_event_reader: EventReader<PrepareChunkLoadEvent>,
    mut chunk_created_event_write: EventWriter<ChunkCreatedEvent>,
    chunk_registry: ResMut<ChunkRegistry>,
) {
    for event in prepare_chunk_load_event_reader.read() {
        let chunk_transform = commands.spawn((
            event.chunk,
            TransformBundle {
                local: {
                    Transform::from_xyz(
                        event.chunk.chunk_x,
                        event.chunk.chunk_y,
                        event.chunk.chunk_z,
                    )
                },
                ..Default::default()
            },
        ));
        chunk_created_event_write.send(ChunkCreatedEvent {
            chunk: event.chunk,
            chunk_id: chunk_transform.id(),
            registry_size: chunk_registry.chunks.len(),
        });
    }
}

pub fn load_chunk_from_queue(
    mut chunk_queue: ResMut<ChunkQueue>,
    mut prepare_chunk_load_event_write: EventWriter<PrepareChunkLoadEvent>,
) {
    let chunks_to_update = 3;
    for _ in 0..chunks_to_update {
        if let Some(chunk) = chunk_queue.chunks.pop() {
            prepare_chunk_load_event_write.send(PrepareChunkLoadEvent {
                chunk: generate_chunk(chunk[0], chunk[1], chunk[2]),
            })
        }
    }
}

// Takes in the chunk_x and chunk_z values to find the chunks
pub fn get_surrounding_chunks(x: i32, _y: i32, z: i32, radius: i32) -> Vec<Vec3> {
    let mut chunks = Vec::new();
    let radius_squared = radius.pow(2);

    for dx in -radius..=radius {
        for dz in -radius..=radius {
            if dx.pow(2) + dz.pow(2) <= radius_squared {
                let chunk_x = x + dx;
                let chunk_z = z + dz;
                // stops chunks above 16 and below 0 from being generated
                // right now we ignore the y value and base it on the chunk vert constant
                // later we probably want a mixture of the two? im not sure but
                // for now this seems to work.
                for chunk_y in 0..CHUNK_VERT {
                    chunks.push(Vec3::new(chunk_x as f32, chunk_y as f32, chunk_z as f32));
                }
            }
        }
    }

    chunks
}

pub fn generate_chunk(x: f32, y: f32, z: f32) -> Chunk {
    let mut chunk = Chunk {
        chunk_x: x,
        chunk_y: y,
        chunk_z: z,
        blocks: [[[Block::default(); CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
    };
    for dx in 0..CHUNK_WIDTH {
        for dy in 0..CHUNK_HEIGHT {
            for dz in 0..CHUNK_DEPTH {
                chunk.blocks[dx][dy][dz] = Block::new(get_random_element(
                    y as usize * 15 + dy,
                    generate_noise(
                        x * 15.0 + dx as f32,
                        y * 15.0 + dy as f32,
                        z * 15.0 + dz as f32,
                    ),
                ));
            }
        }
    }
    chunk
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

pub fn generate_noise(x: f32, y: f32, z: f32) -> f64 {
    let perlin = Perlin::new(1);
    perlin.get([x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1])
}

fn get_random_element(y: usize, noise_value: f64) -> Element {
    // Adjust y based on noise (this is just an example, adjust as needed)
    let adjusted_y = y as f64 + noise_value * 10.0;

    match adjusted_y as usize {
        _ if adjusted_y > 75.0 => Element::Air,
        _ if adjusted_y <= 50.0 && adjusted_y > 5.0 => Element::Dirt,
        _ if adjusted_y <= 25.0 => Element::Stone,
        _ => Element::Grass,
    }
}

fn convert_to_chunk_location(location: f32) -> i32 {
    (location / 16.0).floor() as i32
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
