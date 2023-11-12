use bevy::prelude::*;
use rand::Rng;

const WORLD_X: i32 = 10;
const WORLD_Y: i32 = 5;
const WORLD_Z: i32 = 10;

#[derive(Component)]
pub struct Voxel {}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_query: Query<Entity, With<Voxel>>,
) {
    for block in block_query.iter() {
        commands.entity(block).despawn();
    }

    let mut rng = rand::thread_rng();
    for x in 0..WORLD_X {
        for y in 0..WORLD_Y {
            for z in 0..WORLD_Z {
                if y >= WORLD_Y - 2 {
                    let random_number: i32 = rng.gen_range(0..100);
                    if random_number % 2 == 0 {
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                ..Default::default()
                            },
                            Voxel {},
                        ));
                    }
                } else {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                            ..Default::default()
                        },
                        Voxel {},
                    ));
                }
            }
        }
    }
}
