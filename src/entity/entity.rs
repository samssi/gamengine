use gl::types::GLuint;
use crate::entity::structures::{Transform, Vector3d};
use crate::graphics::opengl::{create_vao};
use crate::state::context::ShaderContext;

pub struct Entity3d {
    pub points: Vec<f32>,
    pub transform: Transform,
    pub program: GLuint,
    pub vao: GLuint
}

fn create_vao_from(context: &ShaderContext, program: &str, points: &Vec<f32>) -> GLuint {
    let program = context.get_program_or_fail(program);
    create_vao(&program, &points)
}

impl Entity3d {
    pub fn with_position(context: &ShaderContext, points: Vec<f32>, program: &str, position: Vector3d) -> Self {
        let vao = create_vao_from(context, program, &points);

        Self {
            points,
            transform: Transform {
                position,
                scale: Vector3d::one_vector(),
                rotation: Vector3d::zero_vector()
            },
            program: context.get_program_or_fail(program),
            vao
        }
    }
    pub fn with_default_transform(context: &ShaderContext, program: &str, points: Vec<f32>) -> Self {
        let vao = create_vao_from(context, program, &points);

        Self {
            points,
            transform: Transform::new_zero_transform(),
            program: context.get_program_or_fail(program),
            vao
        }
    }
}