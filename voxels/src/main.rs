use bevy::prelude::*;
use player::PlayerPlugin;

use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use coordinates::CoordinatePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

mod player;
mod coordinates;
mod main_menu;
mod systems;
mod world;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CoordinatePlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_pause_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Paused,
    #[default]
    Game,
    GameOver,
}
