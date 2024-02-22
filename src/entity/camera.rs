use crate::entity::structures::{Transform, Vector3d};

#[derive(Debug)]
pub struct Camera {
    pub target: Vector3d,
    pub transform: Transform
}