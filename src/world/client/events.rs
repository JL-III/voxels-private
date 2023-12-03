use bevy::prelude::*;

use crate::world::{block::VertexScale, events::RenderChunk};

use super::mesh_utils::{gen_meshes, merge_meshes};

// this needs a better function name
pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_chunk_event_reader: EventReader<RenderChunk>,
    vertex_scale: Res<VertexScale>,
    asset_server: Res<AssetServer>,
) {
    for chunk_event in render_chunk_event_reader.read() {
        let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");
        let combined_mesh = merge_meshes(gen_meshes(vertex_scale.scale, &chunk_event.chunk));
        commands
            .spawn((
                chunk_event.chunk,
                TransformBundle {
                    local: {
                        Transform::from_xyz(
                            chunk_event.chunk.chunk_x,
                            chunk_event.chunk.chunk_y,
                            chunk_event.chunk.chunk_z,
                        )
                    },
                    ..Default::default()
                },
            ))
            .insert(PbrBundle {
                mesh: meshes.add(combined_mesh.clone()),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(block_atlas.clone()),
                    unlit: false,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    chunk_event.chunk.chunk_x,
                    chunk_event.chunk.chunk_y,
                    chunk_event.chunk.chunk_z,
                ),
                ..default()
            });
    }
}
