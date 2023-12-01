use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub position: Vec3,
    pub entity_id: Entity,
}
