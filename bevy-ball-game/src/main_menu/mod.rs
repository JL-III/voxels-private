mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use self::systems::layout::{spawn_main_menu, despawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

