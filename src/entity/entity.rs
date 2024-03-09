use gl::types::GLuint;
use image::DynamicImage;
use crate::entity::structures::{Transform, Vector3d};
use crate::graphics::opengl::{create_vao, create_vao_with_textures};
use crate::state::context::{ObjectContext, ShaderContext};

pub struct Entity3d {
    pub points: Vec<f32>,
    pub transform: Transform,
    pub program: GLuint,
    pub vao: GLuint
}

fn create_vao_from(shader_context: &ShaderContext, texture: &DynamicImage, program: &str, points: &Vec<f32>, texture_coordinates: &Vec<f32>) -> GLuint {
    let program = shader_context.get_program_or_fail(program);
    create_vao_with_textures(&program, &points, texture, texture_coordinates)
}

impl Entity3d {
    pub fn with_transform(
        shader_context: &ShaderContext,
        texture: &DynamicImage,
        program: &str,
        points: Vec<f32>,
        transform: Transform,
        texture_coordinates: Vec<f32>) -> Self {
        let vao = create_vao_from(shader_context, &texture, program, &points, &texture_coordinates);

        Self {
            points,
            transform,
            program: shader_context.get_program_or_fail(program),
            vao
        }
    }

    pub fn with_position_and_scale(
        shader_context: &ShaderContext,
        texture: &DynamicImage,
        points: Vec<f32>,
        texture_coordinates: Vec<f32>,
        program: &str,
        position: Vector3d,
        scale: Vector3d
        ) -> Self {
        let vao = create_vao_from(shader_context, texture, program, &points, &texture_coordinates);

        Self {
            points,
            transform: Transform {
                position,
                scale,
                rotation: Vector3d::zero_vector()
            },
            program: shader_context.get_program_or_fail(program),
            vao
        }
    }
    pub fn with_default_transform(
        shader_context: &ShaderContext,
        texture: &DynamicImage,
        program: &str,
        points: Vec<f32>,
        texture_coordinates: Vec<f32>) -> Self {
        let vao = create_vao_from(shader_context, &texture, program, &points, &texture_coordinates);

        Self {
            points,
            transform: Transform::new_zero_transform(),
            program: shader_context.get_program_or_fail(program),
            vao
        }
    }
}