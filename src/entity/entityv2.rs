use crate::entity::structures::{Transform};
use crate::graphics::object::EntityData;
use crate::graphics::openglv2::{Program, Vao};

pub struct Entity3d {
    pub vao: Vao,
    pub program: Program,
    pub transform: Transform,
    pub entity_data: EntityData,
}

impl Entity3d {
    pub fn create(
        program: Program,
        vao: Vao,
        transform: Transform,
        entity_data: EntityData
    ) -> Self {
        Self {
            vao,
            program,
            transform,
            entity_data
        }
    }
}
