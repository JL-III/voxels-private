use bevy::prelude::*;

use super::debug_ui::{player_move_event_listener, spawn_coordinate_display, spawn_chunk_registry_display, chunk_created_listener};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display)
            .add_systems(Startup, spawn_chunk_registry_display)
            .add_systems(Update, chunk_created_listener)
            .add_systems(Update, player_move_event_listener);
    }
}
