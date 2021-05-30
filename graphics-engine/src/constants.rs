use crate::types::*;

pub const default_matrix4x4: Matrix4x4 = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];
pub const default_vector4: Vector4 = [0.0, 0.0, 0.0, 0.0];

pub const vertex_size: i32 = 4;
pub const triangle_verticies: i32 = 3;
pub const mesh_compression_factor: f32 = 8.0;
pub const max_radius: f32 = 1000.0;
pub const rectangle_array: [f32; (vertex_size * triangle_verticies * 2) as usize] = [
    -1.0, -1.0, 1.0, 1.0,
    1.0, -1.0, 1.0, 1.0,
    -1.0, 1.0, 1.0, 1.0, // Triangle 1
    1.0, -1.0, 1.0, 1.0,
    -1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, // Triangle 2
];
