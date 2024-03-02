use crate::entity::structures::{Transform, Vector3d};

#[derive(Debug)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub distance: f32,
    pub transform: Transform
}