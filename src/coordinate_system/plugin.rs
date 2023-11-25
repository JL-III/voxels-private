use bevy::prelude::*;

use super::coordinates::{player_move_event_listener, spawn_coordinate_display};

pub struct CoordinatePlugin;

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display)
            .add_systems(Update, player_move_event_listener);
    }
}
