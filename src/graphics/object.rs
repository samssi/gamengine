use crate::io::loader::read_wavefront_object_file;
use crate::io::wavefront_reader::{get_geometric_vertices, get_normals, get_texture_points};

pub struct EntityData {
    pub vertices: Vec<f32>,
    pub texture_points: Vec<f32>,
    pub normals: Vec<f32>
}

impl EntityData {
    pub fn create(
        vertices: Vec<f32>,
        texture_points: Vec<f32>,
        normals: Vec<f32>
    ) -> Self {
        Self {
            vertices,
            texture_points,
            normals
        }
    }

    pub fn from_wavefront_object_file(wavefront_obj_file_path: &str) -> Self {
        let wavefront_file_content = read_wavefront_object_file(wavefront_obj_file_path);

        Self::create(
            get_geometric_vertices(&wavefront_file_content),
            get_texture_points(&wavefront_file_content),
            get_normals(&wavefront_file_content)
        )
    }
}