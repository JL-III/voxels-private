use bevy::prelude::*;

pub fn create_block() -> Mesh {
  Mesh::new(create_simple_quad)
}