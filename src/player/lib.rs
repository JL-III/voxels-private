use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_atmosphere::prelude::*;
use bevy_renet::renet::RenetClient;

use crate::{command_system::events::CommandDispatchEvent, ClientChannel, PlayerInput};

use super::events::{PlayerMoveEvent, PlayerSpawnEvent};

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

pub fn grab_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
}

pub fn release_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn initial_grab_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(_window) = window_query.get_single_mut() {
        grab_cursor(window_query);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!")
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut player_spawned_event_writer: EventWriter<PlayerSpawnEvent>,
) {
    let translation = Vec3::new(0.0, 74.0, 0.0);
    let player = commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::new(0.0, 74.0, 1.0), Vec3::Y),
            ..Default::default()
        },
        Player,
        AtmosphereCamera::default(),
    ));
    player_spawned_event_writer.send(PlayerSpawnEvent {
        position: translation,
        entity_id: player.id(),
    });
}

pub fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut client: ResMut<RenetClient>,
) {
    if let Ok(window) = window_query.get_single() {
        for mut transform in transform_query.iter_mut() {
            let mut player_move_event = PlayerMoveEvent {
                starting_position: transform.translation,
                final_position: transform.translation,
            };
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

            player_move_event.final_position = transform.translation;
            if player_move_event.starting_position != player_move_event.final_position {
                let player_input = convert_player_move_event(&player_move_event);
                player_move_event_writer.send(player_move_event);
                let player_input_message = bincode::serialize(&player_input).unwrap();
                client.send_message(ClientChannel::Input, player_input_message);
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!")
    }
}

fn convert_player_move_event(player_move_event: &PlayerMoveEvent) -> PlayerInput {
    PlayerInput {
        up: player_move_event.starting_position.y < player_move_event.final_position.y,
        down: player_move_event.starting_position.y > player_move_event.final_position.y,
        left: player_move_event.starting_position.x < player_move_event.final_position.x,
        right: player_move_event.starting_position.x > player_move_event.final_position.x,
        forward: player_move_event.starting_position.z < player_move_event.final_position.z,
        backward: player_move_event.starting_position.z > player_move_event.final_position.z,
    }
}

pub fn player_look(
    settings: Res<MovementSettings>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut input_state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(window) = window_query.get_single() {
        let delta_state = input_state.as_mut();
        for mut transform in transform_query.iter_mut() {
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

pub fn speed_command(
    mut command_dispatch_event_reader: EventReader<CommandDispatchEvent>,
    mut player_settings: ResMut<MovementSettings>,
) {
    for event in command_dispatch_event_reader.read() {
        let parts: Vec<&str> = event.command.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == "/speed" {
            match parts[1].parse::<f32>() {
                Ok(parsed_value) => {
                    println!("'{}' is a valid f32.", parts[1]);
                    player_settings.speed = parsed_value;
                }
                Err(_) => println!("'{}' is not a valid f32.", parts[1]),
            }
        }
    }
}

pub fn teleport_command(
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut command_dispatch_event_reader: EventReader<CommandDispatchEvent>,
) {
    for event in command_dispatch_event_reader.read() {
        let mut transform = transform_query.single_mut();
        let parts: Vec<&str> = event.command.split_whitespace().collect();
        if parts.len() == 4 && parts[0] == "/tppos" {
            match (
                parts[1].parse::<f32>(),
                parts[2].parse::<f32>(),
                parts[3].parse::<f32>(),
            ) {
                (Ok(x), Ok(y), Ok(z)) => {
                    println!("'{} {} {}' are valid f32s.", x, y, z);
                    transform.translation = Vec3::new(x, y, z);
                }
                _ => println!("'{:?}' is not a valid f32.", parts),
            }
        }
    }
}
