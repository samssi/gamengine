use gl::types::GLuint;
use crate::graphics::opengl::{create_program, create_vao, link_shaders};
use crate::state::context::ShaderContext;

pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const TRIANGLE: [f32; 9] =
    [
        0.0, -100.0, 0.0,
        150.0,  125.0, 0.0,
        -175.0,  100.0, 0.0
    ];

pub fn f_letter_entity() -> Vec<f32> {
    vec![
        // left column front
        0.0, 0.0, 0.0,
        0.0, 150.0, 0.0,
        30.0, 0.0, 0.0,
        0.0, 150.0, 0.0,
        30.0, 150.0, 0.0,
        30.0, 0.0, 0.0,

        // top rung front
        30.0, 0.0, 0.0,
        30.0, 30.0, 0.0,
        100.0, 0.0, 0.0,
        30.0, 30.0, 0.0,
        100.0, 30.0, 0.0,
        100.0, 0.0, 0.0,

        // middle rung front
        30.0, 60.0, 0.0,
        30.0, 90.0, 0.0,
        67.0, 60.0, 0.0,
        30.0, 90.0, 0.0,
        67.0, 90.0, 0.0,
        67.0, 60.0, 0.0,

        // left column back
        0.0, 0.0, 30.0,
        30.0, 0.0, 30.0,
        0.0, 150.0, 30.0,
        0.0, 150.0, 30.0,
        30.0, 0.0, 30.0,
        30.0, 150.0, 30.0,

        // top rung back
        30.0, 0.0, 30.0,
        100.0, 0.0, 30.0,
        30.0, 30.0, 30.0,
        30.0, 30.0, 30.0,
        100.0, 0.0, 30.0,
        100.0, 30.0, 30.0,

        // middle rung back
        30.0, 60.0, 30.0,
        67.0, 60.0, 30.0,
        30.0, 90.0, 30.0,
        30.0, 90.0, 30.0,
        67.0, 60.0, 30.0,
        67.0, 90.0, 30.0,

        // top
        0.0, 0.0, 0.0,
        100.0, 0.0, 0.0,
        100.0, 0.0, 30.0,
        0.0, 0.0, 0.0,
        100.0, 0.0, 30.0,
        0.0, 0.0, 30.0,

        // top rung right
        100.0, 0.0, 0.0,
        100.0, 30.0, 0.0,
        100.0, 30.0, 30.0,
        100.0, 0.0, 0.0,
        100.0, 30.0, 30.0,
        100.0, 0.0, 30.0,

        // under top rung
        30.0, 30.0, 0.0,
        30.0, 30.0, 30.0,
        100.0, 30.0, 30.0,
        30.0, 30.0, 0.0,
        100.0, 30.0, 30.0,
        100.0, 30.0, 0.0,

        // between top rung and middle
        30.0, 30.0, 0.0,
        30.0, 60.0, 30.0,
        30.0, 30.0, 30.0,
        30.0, 30.0, 0.0,
        30.0, 60.0, 0.0,
        30.0, 60.0, 30.0,

        // top of middle rung
        30.0, 60.0, 0.0,
        67.0, 60.0, 30.0,
        30.0, 60.0, 30.0,
        30.0, 60.0, 0.0,
        67.0, 60.0, 0.0,
        67.0, 60.0, 30.0,

        // right of middle rung
        67.0, 60.0, 0.0,
        67.0, 90.0, 30.0,
        67.0, 60.0, 30.0,
        67.0, 60.0, 0.0,
        67.0, 90.0, 0.0,
        67.0, 90.0, 30.0,

        // bottom of middle rung.
        30.0, 90.0, 0.0,
        30.0, 90.0, 30.0,
        67.0, 90.0, 30.0,
        30.0, 90.0, 0.0,
        67.0, 90.0, 30.0,
        67.0, 90.0, 0.0,

        // right of bottom
        30.0, 90.0, 0.0,
        30.0, 150.0, 30.0,
        30.0, 90.0, 30.0,
        30.0, 90.0, 0.0,
        30.0, 150.0, 0.0,
        30.0, 150.0, 30.0,

        // bottom
        0.0, 150.0, 0.0,
        0.0, 150.0, 30.0,
        30.0, 150.0, 30.0,
        0.0, 150.0, 0.0,
        30.0, 150.0, 30.0,
        30.0, 150.0, 0.0,

        // left side
        0.0, 0.0, 0.0,
        0.0, 0.0, 30.0,
        0.0, 150.0, 30.0,
        0.0, 0.0, 0.0,
        0.0, 150.0, 30.0,
        0.0, 150.0, 0.0,
    ]
}


pub struct Transform {
    pub position: Vector3d,
    pub rotation: Vector3d,
    pub scale: Vector3d
}

fn zero_vector() -> Vector3d {
    Vector3d {
        x: 0.0,
        y: 0.0,
        z: 0.0
    }
}

fn one_vector() -> Vector3d {
    Vector3d {
        x: 1.0,
        y: 1.0,
        z: 1.0
    }
}

pub struct Entity3d {
    pub points: Vec<f32>,
    pub transform: Transform,
    pub program: GLuint,
    pub vao: GLuint
}

fn get_program_or_fail(context: &ShaderContext, program: &str) -> GLuint {
    context.programs.get(program).unwrap().clone()
}

fn create_vao_from(context: &ShaderContext, program: &str, points: &Vec<f32>) -> GLuint {
    let program = get_program_or_fail(context, program);
    create_vao(&program, &points)
}

impl Entity3d {
    pub fn with_position(context: &ShaderContext, points: Vec<f32>, program: &str, position: Vector3d) -> Self {
        let vao = create_vao_from(context, program, &points);

        Self {
            points,
            transform: Transform {
                position,
                scale: one_vector(),
                rotation: zero_vector()
            },
            program: get_program_or_fail(context, program),
            vao
        }
    }
    pub fn with_default_transform(context: &ShaderContext, program: &str, points: Vec<f32>) -> Self {
        let vao = create_vao_from(context, program, &points);

        Self {
            points,
            transform: Transform {
                position: zero_vector(),
                scale: one_vector(),
                rotation: zero_vector()
            },
            program: get_program_or_fail(context, program),
            vao
        }
    }
}