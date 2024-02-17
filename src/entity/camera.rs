use crate::entity::structures::{Transform, Vector3d};

pub struct Camera {
    pub target: Vector3d,
    pub transform: Transform
}