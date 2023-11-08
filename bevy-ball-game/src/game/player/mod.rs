use bevy::prelude::*;

use crate::AppState;

use self::systems::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player, despawn_player,
};

use super::SimulationState;

pub mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(ConfinementSystemSet.after(MovementSystemSet))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(player_movement
                .in_set(MovementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)))
            .add_system(confine_player_movement
                .in_set(ConfinementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)))
            .add_systems(
                (
                    enemy_hit_player,
                    player_hit_star
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
