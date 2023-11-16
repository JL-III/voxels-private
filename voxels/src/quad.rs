use bevy::math::{Vec2, Vec3};
use bevy::render::mesh::Indices;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use std::collections::HashMap;
use std::collections::HashSet;
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
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut points_order: HashMap<VertexData, u32> = HashMap::new();
    let mut points_hash: HashSet<VertexData> = HashSet::new();
    let mut tris: Vec<u32> = Vec::new();
    let mut p_index = 0u32;

    for mesh in meshes.iter() {
        if let Some(VertexAttributeValues::Float32x3(vertices)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            if let Some(VertexAttributeValues::Float32x3(normals)) =
                mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            {
                if let Some(VertexAttributeValues::Float32x2(uvs)) =
                    mesh.attribute(Mesh::ATTRIBUTE_UV_0)
                {
                    for ((&[vx, vy, vz], &[nx, ny, nz]), &[ux, uy]) in
                        vertices.iter().zip(normals.iter()).zip(uvs.iter())
                    {
                        let vertex_data = VertexData {
                            position: Vec3::new(vx, vy, vz),
                            normal: Vec3::new(nx, ny, nz),
                            uv: Vec2::new(ux, uy),
                        };
                        if !points_hash.contains(&vertex_data) {
                            points_order.insert(vertex_data.clone(), p_index);
                            points_hash.insert(vertex_data.clone());
                            p_index += 1;
                        }
                    }
                }
            }
        }
        if let Some(indices) = mesh.indices() {
            match indices {
                Indices::U16(indices) => {
                    for &index in indices.iter() {
                        process_vertex_data(
                            mesh,
                            index as u32,
                            &mut points_order,
                            &mut points_hash,
                            &mut tris,
                            &mut p_index,
                        );
                    }
                }
                Indices::U32(indices) => {
                    for &index in indices.iter() {
                        process_vertex_data(
                            mesh,
                            index,
                            &mut points_order,
                            &mut points_hash,
                            &mut tris,
                            &mut p_index,
                        );
                    }
                }
            }
        }
    }

    // Now extract the arrays and assign to the mesh
    extract_arrays(&points_order, &mut mesh);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(tris)));
    mesh
}

fn process_vertex_data(
    mesh: &Mesh,
    index: u32,
    points_order: &mut HashMap<VertexData, u32>,
    points_hash: &mut HashSet<VertexData>,
    tris: &mut Vec<u32>,
    p_index: &mut u32,
) {
    let vertex = if let Some(VertexAttributeValues::Float32x3(vertices)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        Vec3::new(
            vertices[index as usize][0],
            vertices[index as usize][1],
            vertices[index as usize][2],
        )
    } else {
        panic!("Expected Vec3 data for position");
    };

    let normal = if let Some(VertexAttributeValues::Float32x3(normals)) =
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
    {
        Vec3::new(
            normals[index as usize][0],
            normals[index as usize][1],
            normals[index as usize][2],
        )
    } else {
        panic!("Expected Vec3 data for normal");
    };

    let uv =
        if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
            Vec2::new(uvs[index as usize][0], uvs[index as usize][1])
        } else {
            panic!("Expected Vec2 data for UV");
        };

    let vertex_data = VertexData {
        position: vertex,
        normal,
        uv,
    };

    if !points_hash.contains(&vertex_data) {
        points_order.insert(vertex_data.clone(), *p_index);
        points_hash.insert(vertex_data.clone());
        *p_index += 1;
    }

    if let Some(&new_index) = points_order.get(&vertex_data) {
        tris.push(new_index);
    }
}

fn extract_arrays(points_order: &HashMap<VertexData, u32>, mesh: &mut Mesh) {
    let mut verts = Vec::new();
    let mut norms = Vec::new();
    let mut uvs = Vec::new();

    // Assuming the keys are stored in the same order they were inserted.
    for vertex_data in points_order.keys() {
        verts.push(vertex_data.position);
        norms.push(vertex_data.normal);
        uvs.push(vertex_data.uv);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, norms);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
}

// use bevy::{
//     prelude::*,
//     render::{mesh::Indices, render_resource::PrimitiveTopology},
// };

// pub fn create_quad(offset: Vec3) -> Mesh {
//     let vertices = vec![
//         Vec3::new(-0.5, -0.5, 0.5) + offset,
//         Vec3::new(0.5, -0.5, 0.5) + offset,
//         Vec3::new(0.5, 0.5, 0.5) + offset,
//         Vec3::new(-0.5, 0.5, 0.5) + offset,
//     ];
//     let uvs = vec![];
//     let normals = vec![];
//     let indices = vec![];

//     Mesh::new(PrimitiveTopology::TriangleList)
//         .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
//         .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
//         .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
//         .with_indices(Some(Indices::U32(indices)))
// }

// each of these vertices occur 3 times
// does this mean we can just ommit these duplicates?
// or is the function dependent on the order the arrays exist in?

// does it matter how these vertices are introduced into the array?
// because if not then we can realize a pattern here

//noticing a pattern with the 4 coordinates
// front back
// share y values, invert x and z

// front
// - - +
// + - +
// + + +
// - + +

// back
// + - -
// - - -
// - + -
// + + -

// left right
// share y values, invert x and z
// left
// - - -
// - - +
// - + +
// + + -

// right
// + - +
// + - -
// + + -
// + + +

// top bottom
// share x values, invert y and z

//can you match with an or clause?

// match side {
//  front or back => {
//
//  }
// }
