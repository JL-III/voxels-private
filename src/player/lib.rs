use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*};

use crate::command_system::events::CommandDispatchEvent;

#[derive(Resource, Default)]
pub struct InputState {
    pub reader_motion: ManualEventReader<MouseMotion>,
    pub pitch: f32,
    pub yaw: f32,
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
pub struct Player {}

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
