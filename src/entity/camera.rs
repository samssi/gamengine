use crate::entity::structures::{Transform, Vector3d};

pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub distance: f32,
    pub transform: Transform
}

impl Camera {
    pub fn create(near: f32, far: f32, distance: f32, position: Vector3d, rotation: Vector3d) -> Self {
        Self {
            near,
            far,
            distance,
            transform: Transform::create_transform_with_position_and_rotation(position, rotation)
        }
    }
}