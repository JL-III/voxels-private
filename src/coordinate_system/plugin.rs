use bevy::prelude::*;

use super::coordinates::spawn_coordinate_display;

pub struct CoordinatePlugin;

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display);
    }
}
