use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_atmosphere::plugin::AtmosphereCamera;

use crate::player::{events::PlayerSpawnEvent, lib::Player};

pub fn setup_client_player(
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
        Player {},
        AtmosphereCamera::default(),
    ));
    player_spawned_event_writer.send(PlayerSpawnEvent {
        position: translation,
        entity_id: player.id(),
    });
}
