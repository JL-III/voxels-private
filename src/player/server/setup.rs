use bevy::prelude::*;

use crate::player::{events::PlayerSpawnEvent, lib::Player};

pub fn setup_server_player(
    mut commands: Commands,
    mut player_spawned_event_writer: EventWriter<PlayerSpawnEvent>,
) {
    let translation = Vec3::new(0.0, 74.0, 0.0);
    let player = commands.spawn((
        Player {},
        Transform {
            translation,
            ..Default::default()
        },
    ));
    player_spawned_event_writer.send(PlayerSpawnEvent {
        position: translation,
        entity_id: player.id(),
    });
}
