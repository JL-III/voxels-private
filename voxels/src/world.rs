use bevy::prelude::*;
use rand::Rng;
use bevy::utils::Duration;
use bevy_atmosphere::prelude::*;

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

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
struct Sun;

// Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
#[derive(Resource)]
struct CycleTimer(Timer);

// We can edit the Atmosphere resource and it will be updated automatically
fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_seconds_wrapped() / 1000.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
        }
    }
}

// Simple environment
fn setup_environment(
    mut commands: Commands,
) {
    // Our Sun
    commands.spawn((
        DirectionalLightBundle {
            ..Default::default()
        },
        Sun, // Marks the light as Sun
    ));
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
        .insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0., 0., -1.),
            rayleigh_coefficient: Vec3::new(1e-5, 1e-5, 1e-5),
            ..default()
        })) // Default Atmosphere material, we can edit it to simulate another planet
        .insert_resource(CycleTimer(Timer::new(
            Duration::from_millis(50), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            TimerMode::Repeating,
        )))
        .add_plugins((AtmospherePlugin,))
        .add_systems(Startup, setup_environment)
        .add_systems(Update, daylight_cycle);
    }
}