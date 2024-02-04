pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Rotation {
// Degress struct
}

pub struct Transform {
    pub position: Vector3d,
    //rotation: Rotation,
    pub scale: Vector3d
}

pub struct Entity3d {
    pub points: Vec<f32>,
    pub transform: Transform
}

impl Entity3d {
    pub fn new(points: Vec<f32>) -> Self {
        Self {
            points,
            transform: Transform {
                position: Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                scale: Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
            }
        }
    }
}