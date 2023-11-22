use crate::{
    block::{create_quad, Block, BlockFace},
    element::Element,
    mesh_utils::merge_meshes, player::PlayerMoveEvent,
};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut},
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

#[derive(Component)]

pub struct Chunk {
    blocks: [[[Block; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
}

fn setup(mut commands: Commands, mut chunk_create_event_write: EventWriter<ChunkCreatedEvent>) {
    let mut chunk = Chunk {
        blocks: [[[Block::default(); CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
    };

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                chunk.blocks[x][y][z] = Block::new(get_random_element(y));
            }
        }
    }

    commands.spawn((
        chunk,
        TransformBundle {
            local: { Transform::from_xyz(0.0, 0.0, 0.0) },
            ..Default::default()
        },
    ));
    chunk_create_event_write.send_default();
}

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_create_event_reader: EventReader<ChunkCreatedEvent>,
    asset_server: Res<AssetServer>,
    transform_query: Query<&Transform, With<Chunk>>,
    chunk_query: Query<(Entity, &Chunk)>,
) {
    for _chunk_event in chunk_create_event_reader.read() {
        let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");

        for transform in transform_query.iter() {
            println!("{}", transform.translation);
            let mut gen_meshes: Vec<Mesh> = Vec::new();

            // Initialize each block
            for (_id, chunk) in chunk_query.iter() {
                println!("{}", chunk.blocks.len());
                for x in 0..CHUNK_WIDTH {
                    for y in 0..CHUNK_HEIGHT {
                        for z in 0..CHUNK_DEPTH {
                            let block = chunk.blocks[x][y][z];
                            if x == CHUNK_WIDTH - 1 {
                                gen_meshes.push(create_quad(
                                    BlockFace::East,
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    block.uv_mapping,
                                ));
                            }
                            if z == CHUNK_DEPTH - 1 {
                                gen_meshes.push(create_quad(
                                    BlockFace::North,
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    block.uv_mapping,
                                ));
                            }
                            if y == CHUNK_HEIGHT - 1 {
                                gen_meshes.push(create_quad(
                                    BlockFace::Top,
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    block.uv_mapping,
                                ));
                            }
                            if y == 0 {
                                gen_meshes.push(create_quad(
                                    BlockFace::Bottom,
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    block.uv_mapping,
                                ));
                            }
                            if z == 0 {
                                gen_meshes.push(create_quad(
                                    BlockFace::South,
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    block.uv_mapping,
                                ));
                            }
                            if x == 0 {
                                gen_meshes.push(create_quad(
                                    BlockFace::West,
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    block.uv_mapping,
                                ));
                            }
                        }
                    }
                }
            }

            let combined_mesh = merge_meshes(gen_meshes);
            commands.spawn((PbrBundle {
                mesh: meshes.add(combined_mesh.clone()),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(block_atlas.clone()),
                    unlit: false,
                    ..default()
                }),
                transform: Transform { ..default() },
                ..default()
            },));
        }
    }
}

fn player_move_event_listener(mut player_move_event_reader: EventReader<PlayerMoveEvent>) {
    for event in player_move_event_reader.read() {
        println!("starting position: {}, final position: {}", event.starting_position, event.final_position);
    }
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
        app.add_event::<ChunkCreatedEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, render)
            .add_systems(Update, player_move_event_listener);
    }
}

#[derive(Event)]
pub struct ChunkCreatedEvent;

impl Default for ChunkCreatedEvent {
    fn default() -> Self {
        Self
    }
}