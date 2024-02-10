use gl::types::GLuint;
use crate::graphics::opengl::link_program;
use crate::state::context::ShaderContext;

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

pub struct Shading {
    pub vertex_shader: String,
    pub fragment_shader: String
}

pub struct Entity3d {
    pub points: Vec<f32>,
    pub program: GLuint,
    pub transform: Transform
}

impl Entity3d {
    pub fn with_default_transform(context: &ShaderContext, points: Vec<f32>, shading: Shading) -> Self {
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
            },
            program: link_program(
                context.vertex_shaders.get(&*shading.vertex_shader)
                    .expect(&format!("failed to load vertex shader: {}", shading.vertex_shader)),
                context.fragment_shaders.get(&*shading.fragment_shader)
                    .expect(&format!("failed to load fragment shader: {}", shading.fragment_shader)))
                .expect(&format!("shader linking failed for vertex shader {} and fragment shader {}", shading.vertex_shader, shading.fragment_shader))
        }
    }
}