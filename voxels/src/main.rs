use bevy::prelude::*;
use camera::PlayerPlugin;

use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use coordinates::CoordinatePlugin;

mod camera;
mod coordinates;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CoordinatePlugin)
        .run();
}
