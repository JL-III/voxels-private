use crate::{
    block::{create_quad, Block, BlockFace},
    element::Element,
    mesh_utils::merge_meshes,
    player::PlayerMoveEvent,
};
use bevy::{
    app::{Plugin, Update},
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::{Commands, Res, ResMut, Resource},
    },
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    render::{mesh::Mesh, texture::Image},
    transform::{components::Transform, TransformBundle},
};

const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;
const CHUNK_DEPTH: usize = 16;

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Chunk {
    chunk_x: isize,
    chunk_z: isize,
    blocks: [[[Block; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
}

fn chunk_enter_listener(
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut commands: Commands,
    mut chunk_enter_event_reader: EventReader<ChunkEnterEvent>,
    mut chunk_create_event_write: EventWriter<ChunkCreatedEvent>,
) {
    for event in chunk_enter_event_reader.read() {
        let mut chunk = Chunk {
            chunk_x: event.chunk_x,
            chunk_z: event.chunk_z,
            blocks: [[[Block::default(); CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
        };
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    chunk.blocks[x][y][z] = Block::new(get_random_element(y));
                }
            }
        }
        if !chunk_registry.chunks.contains(&chunk) {
            chunk_registry.chunks.push(chunk);
            commands.spawn((
                chunk,
                TransformBundle {
                    local: { Transform::from_xyz(chunk.chunk_x as f32, 0.0, chunk.chunk_z as f32) },
                    ..Default::default()
                },
            ));
            chunk_create_event_write.send(ChunkCreatedEvent { chunk });
        }
    }
}

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_create_event_reader: EventReader<ChunkCreatedEvent>,
    asset_server: Res<AssetServer>,
) {
    for chunk_event in chunk_create_event_reader.read() {
        let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");

        let combined_mesh = merge_meshes(gen_meshes(chunk_event));
        commands.spawn(PbrBundle {
            mesh: meshes.add(combined_mesh.clone()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(block_atlas.clone()),
                unlit: false,
                ..default()
            }),
            transform: Transform::from_xyz(
                chunk_event.chunk.chunk_x as f32,
                0.0,
                chunk_event.chunk.chunk_z as f32,
            ),
            ..default()
        });
    }
}

fn player_move_event_listener(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut enter_chunk_event_writer: EventWriter<ChunkEnterEvent>,
) {
    for event in player_move_event_reader.read() {
        let starting_chunk_x = (event.starting_position.x / 16.0).floor() as isize;
        let starting_chunk_z = (event.starting_position.z / 16.0).floor() as isize;

        let final_chunk_x = (event.final_position.x / 16.0).floor() as isize;
        let final_chunk_z = (event.final_position.z / 16.0).floor() as isize;

        if starting_chunk_x != final_chunk_x || starting_chunk_z != final_chunk_z {
            println!("Inside chunk: x: {} z: {}", final_chunk_x, final_chunk_z);
            enter_chunk_event_writer.send(ChunkEnterEvent {
                chunk_x: final_chunk_x,
                chunk_z: final_chunk_z,
            });
        }
    }
}

fn gen_meshes(chunk_event: &ChunkCreatedEvent) -> Vec<Mesh> {
    let mut gen_meshes: Vec<Mesh> = Vec::new();

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                let block = chunk_event.chunk.blocks[x][y][z];
                let mesh_location = Vec3::new(
                    chunk_event.chunk.chunk_x as f32 * 16.0 + x as f32,
                    y as f32,
                    chunk_event.chunk.chunk_z as f32 * 16.0 + z as f32,
                );
                if x == CHUNK_WIDTH - 1 {
                    gen_meshes.push(create_quad(
                        BlockFace::East,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if z == CHUNK_DEPTH - 1 {
                    gen_meshes.push(create_quad(
                        BlockFace::North,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if y == CHUNK_HEIGHT - 1 {
                    gen_meshes.push(create_quad(BlockFace::Top, mesh_location, block.uv_mapping));
                }
                if y == 0 {
                    gen_meshes.push(create_quad(
                        BlockFace::Bottom,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if z == 0 {
                    gen_meshes.push(create_quad(
                        BlockFace::South,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if x == 0 {
                    gen_meshes.push(create_quad(
                        BlockFace::West,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
            }
        }
    }
    gen_meshes
}

fn get_random_element(y: usize) -> Element {
    match y {
        _ if y == 10 => Element::Grass,
        _ if y < 10 && y > 5 => Element::Dirt,
        _ if y <= 5 => Element::Stone,
        _ => Element::Air,
    }
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ChunkRegistry {
            chunks: Vec::<Chunk>::new(),
        })
        .add_event::<ChunkCreatedEvent>()
        .add_event::<ChunkEnterEvent>()
        .add_systems(Update, chunk_enter_listener)
        .add_systems(Update, render)
        .add_systems(Update, player_move_event_listener);
    }
}

#[derive(Event)]
pub struct ChunkCreatedEvent {
    pub chunk: Chunk,
}

#[derive(Event)]
pub struct ChunkEnterEvent {
    pub chunk_x: isize,
    pub chunk_z: isize,
}

#[derive(Resource)]
pub struct ChunkRegistry {
    pub chunks: Vec<Chunk>,
}
