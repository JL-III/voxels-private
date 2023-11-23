#![deny(clippy::unwrap_used)]
use app_state::{plugin::AppStatePlugin, state::AppState};
use bevy::prelude::*;

use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use command_system::plugin::CommandPlugin;
use coordinate_system::plugin::CoordinatePlugin;
use main_menu::plugin::MainMenuPlugin;
use player::plugin::PlayerPlugin;
use world::plugin::WorldPlugin;

mod app_state;
mod command_system;
mod coordinate_system;
mod main_menu;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(AppStatePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(CoordinatePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(CommandPlugin)
        .run();
}
