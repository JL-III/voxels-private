use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    coordinates::CoordinateDisplay,
    world::{setup_world, Voxel},
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
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

#[derive(Component)]
pub struct FlyCam;

pub fn toggle_grap_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub fn initial_grab_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        toggle_grap_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!")
    }
}

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
    ));
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<FlyCam>>,
    mut coordinate_display_query: Query<&mut Text, With<CoordinateDisplay>>,
) {
    if let Ok(window) = window_query.get_single() {
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
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
                    "x: {}, y: {}, z: {}",
                    transform.translation.x as i32,
                    transform.translation.y as i32,
                    transform.translation.z as i32
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
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = window_query.get_single() {
        let delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.read(&motion) {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        delta_state.pitch -=
                            (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        delta_state.yaw -=
                            (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }
                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    }
}

pub fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grap_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`");
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_systems(Startup, setup_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, cursor_grab)
            .add_systems(Update, run_world_gen);
    }
}
