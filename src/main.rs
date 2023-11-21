#![deny(clippy::unwrap_used)]
use bevy::prelude::*;
use player::PlayerPlugin;

use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use command_interface::CommandPlugin;
use coordinates::CoordinatePlugin;
use main_menu::MainMenuPlugin;
use systems::*;
use world::WorldPlugin;

mod block;
mod command_interface;
mod coordinates;
mod main_menu;
mod element;
mod mesh_utils;
mod player;
mod systems;
mod world;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(CoordinatePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(CommandPlugin)
        .add_systems(
            Update,
            transition_to_game_state.run_if(in_state(AppState::Paused)),
        )
        .add_systems(
            Update,
            transition_to_game_state.run_if(in_state(AppState::Command)),
        )
        .add_systems(
            Update,
            transition_to_pause_state.run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            transition_to_command_state.run_if(in_state(AppState::Game)),
        )
        .run();
}

//pausing will only stop the player control, the game will continue on
//the same applies to the command state
//i dont think there will ever be a game over state, that might just be used for a death screen

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Paused,
    Command,
    #[default]
    Game,
    GameOver,
}
