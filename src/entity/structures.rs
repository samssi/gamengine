#[derive(Debug, Clone)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3d {
    pub fn zero_vector() -> Vector3d {
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
    pub fn one_vector() -> Vector3d {
        Vector3d {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3d,
    pub rotation: Vector3d,
    pub scale: Vector3d
}

impl Transform {
     pub fn new_zero_transform() -> Self {
         Transform {
             position: Vector3d::zero_vector(),
             rotation: Vector3d::zero_vector(),
             scale: Vector3d::one_vector(),
         }
     }
    pub fn new_zero_transform_with_position(position: Vector3d) -> Self {
        Transform {
            position,
            rotation: Vector3d::zero_vector(),
            scale: Vector3d::one_vector(),
        }
    }
    pub fn new_transform_with_position_and_rotation(position: Vector3d, rotation: Vector3d) -> Self {
        Transform {
            position,
            rotation,
            scale: Vector3d::one_vector(),
        }
    }
    pub fn new_transform_with_position_rotation_and_scale(position: Vector3d, rotation: Vector3d, scale: Vector3d) -> Self {
        Transform {
            position,
            rotation,
            scale,
        }
    }
}