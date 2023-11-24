use std::ops::Index;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use crate::{command_system::events::CommandDispatchEvent, player::controls::Player};

use super::{element::Element, mesh_utils::merge_meshes};

#[derive(Clone, Copy)]
pub enum BlockFace {
    North,
    South,
    East,
    West,
    Top,
    Bottom,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Block {
    pub uv_mapping: UVMapping,
    pub element: Element,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            uv_mapping: UVMapping([0.0, 0.0]),
            element: Element::Air,
        }
    }
}

impl Block {
    pub fn new(element: Element) -> Self {
        let uv_mapping = match element {
            Element::Air => UVMapping([3.0, 6.0]),
            Element::Stone => UVMapping([0.0, 1.0]),
            Element::Dirt => UVMapping([0.0, 2.0]),
            Element::Grass => UVMapping([0.0, 3.0]),
        };

        Self {
            uv_mapping,
            element,
        }
    }
}

#[derive(Component, Copy, Debug, PartialEq)]
pub struct UVMapping(pub [f32; 2]);

impl Clone for UVMapping {
    fn clone(&self) -> Self {
        *self
    }
}

impl Index<usize> for UVMapping {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Resource)]
pub struct VertexScale {
    pub scale: f32,
}

pub fn create_quad(scale: f32, side: BlockFace, offset: Vec3, uv_mapping: UVMapping) -> Mesh {
    let result = match side {
        BlockFace::North => create_north_quad(scale, offset, uv_mapping),
        BlockFace::South => create_south_quad(scale, offset, uv_mapping),
        BlockFace::West => create_west_quad(scale, offset, uv_mapping),
        BlockFace::East => create_east_quad(scale, offset, uv_mapping),
        BlockFace::Top => create_top_quad(scale, offset, uv_mapping),
        BlockFace::Bottom => create_bottom_quad(scale, offset, uv_mapping),
    };

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, result.0)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, result.1)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, result.2)
}

pub fn create_north_quad(
    scale: f32,
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(0.0, 0.0, scale) + offset,
        Vec3::new(scale, 0.0, scale) + offset,
        Vec3::new(scale, scale, scale) + offset,
        Vec3::new(0.0, scale, scale) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_south_quad(
    scale: f32,
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(scale, 0.0, 0.0) + offset,
        Vec3::new(0.0, 0.0, 0.0) + offset,
        Vec3::new(0.0, scale, 0.0) + offset,
        Vec3::new(scale, scale, 0.0) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_west_quad(
    scale: f32,
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(0.0, 0.0, 0.0) + offset,
        Vec3::new(0.0, 0.0, scale) + offset,
        Vec3::new(0.0, scale, scale) + offset,
        Vec3::new(0.0, scale, 0.0) + offset,
    ];
    let normals = vec![
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_east_quad(
    scale: f32,
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(scale, 0.0, scale) + offset,
        Vec3::new(scale, 0.0, 0.0) + offset,
        Vec3::new(scale, scale, 0.0) + offset,
        Vec3::new(scale, scale, scale) + offset,
    ];
    let normals = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_top_quad(
    scale: f32,
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(0.0, scale, scale) + offset,
        Vec3::new(scale, scale, scale) + offset,
        Vec3::new(scale, scale, 0.0) + offset,
        Vec3::new(0.0, scale, 0.0) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_bottom_quad(
    scale: f32,
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(0.0, 0.0, 0.0) + offset,
        Vec3::new(scale, 0.0, 0.0) + offset,
        Vec3::new(scale, 0.0, scale) + offset,
        Vec3::new(0.0, 0.0, scale) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];

    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

// this function takes the row and column index to identify its texture location in the block atlas
pub fn get_texture(row: f32, column: f32) -> Vec<Vec2> {
    let grid_size = 16.0;
    let mut uvs: Vec<Vec2> = Vec::new();

    let left = column / grid_size;
    let right = (column + 1.0) / grid_size;
    let bottom = row / grid_size;
    let top = (row + 1.0) / grid_size;
    uvs.push(Vec2::new(left, bottom));
    uvs.push(Vec2::new(right, bottom));
    uvs.push(Vec2::new(right, top));
    uvs.push(Vec2::new(left, top));

    uvs
}

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    vertex_scale: Res<VertexScale>,
    asset_server: Res<AssetServer>,
    mut command_dispatch_event_reader: EventReader<CommandDispatchEvent>,
    transform_query: Query<&Transform, With<Player>>,
) {
    for event in command_dispatch_event_reader.read() {
        let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");
        let sides = vec![
            BlockFace::Top,
            BlockFace::Bottom,
            BlockFace::East,
            BlockFace::West,
            BlockFace::North,
            BlockFace::South,
        ];
        let parts: Vec<&str> = event.command.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == "/block" {
            let mut element = Element::Air;
            match Element::from_str(parts[1]) {
                Some(Element::Air) => {}
                Some(Element::Dirt) => element = Element::Dirt,
                Some(Element::Grass) => element = Element::Grass,
                Some(Element::Stone) => element = Element::Stone,
                _ => {
                    println!("'{}' is not a valid element.", parts[1]);
                    return;
                }
            }
            if let Ok(transform) = transform_query.get_single() {
                println!("inside transform");
                let mut combined_mesh: Vec<Mesh> = Vec::new();
                let block = Block::new(element);
                for side in sides {
                    combined_mesh.push(create_quad(
                        vertex_scale.scale,
                        side,
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        block.uv_mapping,
                    ));
                }
                println!("number of meshes in combined_mesh: {}", combined_mesh.len());
                println!(
                    "transform translation  x: {}, y: {}, z: {}",
                    transform.translation.x, transform.translation.y, transform.translation.z,
                );
                commands.spawn(PbrBundle {
                    mesh: meshes.add(merge_meshes(combined_mesh)),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(block_atlas.clone()),
                        unlit: false,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z + 1.0,
                    ),
                    ..default()
                });
            } else {
                warn!("player transform not found!");
            }
        }
    }
}
