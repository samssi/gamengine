use crate::entity::structures::{Transform, Vector3d};
use crate::graphics::openglv2::{Program, Vao};

pub struct Entity3d {
    pub vao: Vao,
    pub program: Program,
    pub transform: Transform,
}

impl Entity3d {
    pub fn create(
        vao: Vao,
        transform: Transform,
        program: Program
    ) -> Self {
        Self {
            vao,
            program,
            transform
        }
    }
}
