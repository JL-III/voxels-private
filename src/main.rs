#![deny(clippy::unwrap_used)]
use app_state::{plugin::AppStatePlugin, state::AppState};
use bevy::prelude::*;

use bevy_debug_grid::DebugGridPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use command_system::plugin::CommandPlugin;
use debug_menu::plugin::DebugPlugin;
use main_menu::plugin::MainMenuPlugin;
use player::plugin::PlayerPlugin;
use world::plugin::WorldPlugin;

mod app_state;
mod command_system;
mod debug_menu;
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
        .add_plugins(DebugPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(CommandPlugin)
        .add_plugins((DebugGridPlugin::with_floor_grid(),))
        .run();
}
