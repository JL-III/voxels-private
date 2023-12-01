use bevy::prelude::*;

#[derive(Event)]
pub struct ClientSentMoveEvent {
    pub direction: Vec3,
}

#[derive(Event)]
pub struct DictatePlayerPositionEvent {
    pub position: Vec3,
}
