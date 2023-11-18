use bevy::math::{Vec2, Vec3};
use bevy::render::mesh::Indices;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone)]
struct VertexData {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl PartialEq for VertexData {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.normal == other.normal && self.uv == other.uv
    }
}

impl Eq for VertexData {}

impl Hash for VertexData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert each floating-point number to its bitwise representation
        // and then hash that representation.
        // This approach treats identical bit patterns as equal.
        hash_float_array(&self.position.to_array(), state);
        hash_float_array(&self.normal.to_array(), state);
        hash_float_array(&self.uv.to_array(), state);
    }
}

fn hash_float_array<H: Hasher>(arr: &[f32], state: &mut H) {
    for &num in arr {
        // We use to_bits to convert the floating-point number to its
        // bitwise representation, which can be hashed.
        num.to_bits().hash(state);
    }
}

pub fn merge_meshes(meshes: Vec<Mesh>) -> Mesh {
    let mut combined_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    
    let mut offset = 0; // This will keep track of the offset for the indices of each quad
    for mesh in meshes {
        // Gather the vertex attributes for positions, normals, and uvs
        if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            vertices.extend_from_slice(positions);
        }
        if let Some(VertexAttributeValues::Float32x3(norms)) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
            normals.extend_from_slice(norms);
        }
        if let Some(VertexAttributeValues::Float32x2(texture_coords)) = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
            uvs.extend_from_slice(texture_coords);
        }
        
        // Adjust the indices for each quad by adding the offset
        if let Some(Indices::U32(mesh_indices)) = mesh.indices() {
            for &index in mesh_indices {
                indices.push(index + offset);
            }
        }
        
        // Increment the offset by the number of vertices in the current quad (which should be 4 for a quad)
        offset += 4;
    }
    
    // Now you can set the vertices, normals, uvs, and indices for the combined mesh
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    combined_mesh.set_indices(Some(Indices::U32(indices)));
    
    combined_mesh
}
