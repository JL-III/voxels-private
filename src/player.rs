use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_atmosphere::prelude::*;

use crate::{
    block::{create_quad, Block, BlockFace},
    coordinates::CoordinateDisplay,
    element::Element,
    mesh_utils::merge_meshes,
    world::{setup_world, Voxel},
    AppState,
};

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00043,
            speed: 12.,
        }
    }
}

#[derive(Component)]
pub struct Player;

fn grab_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
        println!("Cursorgrab set to Confined")
    }
}

fn release_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        println!("Cursorgrab set to None")
    }
}

pub fn initial_grab_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(_window) = window_query.get_single_mut() {
        grab_cursor(window_query);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!")
    }
}

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
            ..Default::default()
        },
        Player,
        AtmosphereCamera::default(),
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<Player>>,
    mut coordinate_display_query: Query<&mut Text, With<CoordinateDisplay>>,
) {
    if let Ok(window) = window_query.get_single() {
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keyboard_input.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => match key {
                        KeyCode::S => velocity += forward,
                        KeyCode::W => velocity -= forward,
                        KeyCode::A => velocity -= right,
                        KeyCode::D => velocity += right,
                        KeyCode::Space => velocity += Vec3::Y,
                        KeyCode::ShiftLeft => velocity -= Vec3::Y,
                        _ => (),
                    },
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * settings.speed;
            for mut text in &mut coordinate_display_query {
                text.sections[0].value = format!(
                    "x: {}, y: {}, z: {}, rotation: {}",
                    transform.translation.x as i32,
                    transform.translation.y as i32,
                    transform.translation.z as i32,
                    transform.rotation
                );
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!")
    }
}

pub fn player_look(
    settings: Res<MovementSettings>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut input_state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(window) = window_query.get_single() {
        let delta_state = input_state.as_mut();
        for mut transform in query.iter_mut() {
            let (current_yaw, current_pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            for ev in delta_state.reader_motion.read(&motion) {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        let delta_pitch =
                            (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        let delta_yaw =
                            (settings.sensitivity * ev.delta.x * window_scale).to_radians();

                        delta_state.pitch = (current_pitch - delta_pitch).clamp(-1.54, 1.54);
                        delta_state.yaw = current_yaw - delta_yaw;

                        transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                            * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
                    }
                }
            }
        }
    }
}

pub fn run_world_gen(
    keys: Res<Input<KeyCode>>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    block_query: Query<Entity, With<Voxel>>,
) {
    if keys.just_pressed(KeyCode::E) {
        setup_world(commands, meshes, materials, block_query);
    }
}

pub fn run_mesh(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let block_faces = [
        BlockFace::North,
        BlockFace::South,
        BlockFace::West,
        BlockFace::East,
        BlockFace::Top,
        BlockFace::Bottom,
    ];

    let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");

    if keys.just_pressed(KeyCode::Numpad0) {
        let mut gen_meshes: Vec<Mesh> = Vec::new();
        for side in block_faces.iter() {
            gen_meshes.push(create_quad(*side, Vec3::new(-1.0, 0.0, 3.0)));
        }

        let block = Block::new(Element::Stone);
        let combined_mesh = merge_meshes(gen_meshes, &block);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(combined_mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(block_atlas),
                    unlit: false,
                    ..default()
                }),
                transform: Transform { ..default() },
                ..default()
            },
            block,
        ));
    } else if keys.just_pressed(KeyCode::Numpad1) {
        let mut gen_meshes: Vec<Mesh> = Vec::new();
        for side in block_faces.iter() {
            gen_meshes.push(create_quad(*side, Vec3::new(0.0, 0.0, 3.0)));
        }

        let block = Block::new(Element::Dirt);
        let combined_mesh = merge_meshes(gen_meshes, &block);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(combined_mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(block_atlas),
                    unlit: false,
                    ..default()
                }),
                transform: Transform { ..default() },
                ..default()
            },
            block,
        ));
    } else if keys.just_pressed(KeyCode::Numpad2) {
        let mut gen_meshes: Vec<Mesh> = Vec::new();
        for side in block_faces.iter() {
            gen_meshes.push(create_quad(*side, Vec3::new(1.0, 0.0, 3.0)));
        }

        let block = Block::new(Element::Grass);
        let combined_mesh = merge_meshes(gen_meshes, &block);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(combined_mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(block_atlas),
                    unlit: false,
                    ..default()
                }),
                transform: Transform { ..default() },
                ..default()
            },
            block,
        ));
    } else if keys.just_pressed(KeyCode::Numpad3) {
        let width = 16;
        let length = 16;
        let height = 16;

        // TODO we will want to generate this mesh based on a query for blocks?

        for x in 0..width {
            for z in 0..length {
                for y in 0..height {
                    let mut gen_meshes: Vec<Mesh> = Vec::new();

                    if x == width - 1 {
                        gen_meshes.push(create_quad(
                            BlockFace::East,
                            Vec3::new(x as f32, y as f32, z as f32),
                        ));
                    }
                    if z == length - 1 {
                        gen_meshes.push(create_quad(
                            BlockFace::North,
                            Vec3::new(x as f32, y as f32, z as f32),
                        ));
                    }
                    if y == height - 1 {
                        gen_meshes.push(create_quad(
                            BlockFace::Top,
                            Vec3::new(x as f32, y as f32, z as f32),
                        ));
                    }
                    if y == 0 {
                        gen_meshes.push(create_quad(
                            BlockFace::Bottom,
                            Vec3::new(x as f32, y as f32, z as f32),
                        ));
                    }
                    if z == 0 {
                        gen_meshes.push(create_quad(
                            BlockFace::South,
                            Vec3::new(x as f32, y as f32, z as f32),
                        ));
                    }
                    if x == 0 {
                        gen_meshes.push(create_quad(
                            BlockFace::West,
                            Vec3::new(x as f32, y as f32, z as f32),
                        ));
                    }

                    let block = Block::new(Element::Grass);
                    let combined_mesh = merge_meshes(gen_meshes, &block);
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
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_systems(Startup, setup_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, run_mesh)
            .add_systems(Update, run_world_gen.run_if(in_state(AppState::Game)))
            .add_systems(
                FixedUpdate,
                (player_move, player_look).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnEnter(AppState::Game), grab_cursor)
            .add_systems(OnExit(AppState::Game), release_cursor);
    }
}
