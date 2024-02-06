pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const TRIANGLE: [f32; 9] =
    [  -0.4, -0.2, 0.0,
        0.4, -0.2, 0.0,
        0.0, 0.2, 0.0
    ];

pub struct Transform {
    pub position: Vector3d,
    pub rotation: Vector3d,
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
                    x: 1.0,
                    y: 1.0,
                    z: 1.0
                },
                rotation: Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                }
            }
        }
    }
}