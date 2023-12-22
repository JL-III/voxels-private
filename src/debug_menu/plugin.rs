use bevy::prelude::*;

use super::debug_ui::{player_move_event_listener, spawn_coordinate_display};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display)
            // These systems will need to be concerned with the players currently rendered chunks
            // so instead of chunk registry it might be something like client chunk registry or rendered chunk registry.
            // .add_systems(Startup, spawn_chunk_registry_display)
            // .add_systems(Update, chunk_created_listener)
            .add_systems(Update, player_move_event_listener);
    }
}
