use bevy::prelude::*;

use self::systems::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player,
};

pub mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]

pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_set(ConfinementSystemSet.after(MovementSystemSet))
        .add_startup_system(spawn_player)
        .add_system(player_movement.in_set(MovementSystemSet))
        .add_system(confine_player_movement.in_set(ConfinementSystemSet))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
