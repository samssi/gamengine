use std::any::Any;
use std::ptr;
use gl::types::{GLenum, GLint, GLuint};
use crate::graphics::opengl_util::{as_c_string, as_c_void, as_const_gluint, as_glsizeiptr, as_stride, get_attrib_location, get_program_compilation_status, get_shader_compilation_status, map_params_to_program};

#[derive(Debug)]
pub enum ShaderType {
    FragmentShader,
    VertexShader
}

pub fn to_gl_shader_type(shader_type: &ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => { gl::VERTEX_SHADER }
        ShaderType::FragmentShader => { gl::FRAGMENT_SHADER }
    }
}

// TODO: fixme
pub fn expect_shader_type_or_panic(shader: &Shader, shader_type: ShaderType) -> Result<(), &str> {
    if matches!(&shader.shader_type, shader_type) {
        Err("Shader type mismatch!")
    }
    else {
        Ok(())
    }
}

pub enum ShaderParamType {
    UniformMat4,
    UniformSampler2d,
    Vec2,
    Vec4
}

pub struct ShaderParam {
    pub attribute_name: String,
    pub param_type: ShaderParamType
}

pub struct Shader {
    pub shader: u32,
    pub shader_type: ShaderType,
    pub shader_source_code: String,
    pub shader_params: Option<Vec<ShaderParam>>
}

impl Shader {
    pub fn create(shader_type: ShaderType, shader_source_code: String, shader_params: Option<Vec<ShaderParam>>) -> Self {
        unsafe {
            let gl_shader_source = as_c_string(&shader_source_code);
            let gl_shader = gl::CreateShader(to_gl_shader_type(&shader_type));

            gl::ShaderSource(gl_shader, 1, &gl_shader_source.as_ptr(), ptr::null());
            gl::CompileShader(gl_shader);
            get_shader_compilation_status(gl_shader).expect("Shader compilation failure!");

            Shader {shader: gl_shader, shader_type, shader_source_code, shader_params}
        }
    }
}



impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.shader as GLuint) }
    }
}

pub struct Program {
    pub program: u32
}

impl Program {
    pub fn create(vertex_shader: &Shader, fragment_shader: &Shader) -> Self {
        let error_message = "Wrong type of shader as parameter";
        // TODO: fixme
        // expect_shader_type_or_panic(&vertex_shader, ShaderType::VertexShader).expect(error_message);
        // expect_shader_type_or_panic(&fragment_shader, ShaderType::FragmentShader).expect(error_message);

        unsafe {
            let gl_program = gl::CreateProgram();
            let gl_vertex_shader: GLuint = vertex_shader.shader as GLuint;
            let gl_fragment_shader: GLuint = fragment_shader.shader as GLuint;

            gl::AttachShader(gl_program, gl_vertex_shader);
            gl::AttachShader(gl_program, gl_fragment_shader);

            gl::LinkProgram(gl_program);
            get_program_compilation_status(gl_program).expect("Shader linking failure while creating program!");

            Program{program: gl_program}
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program as GLuint) }
    }
}

pub struct Vao {
    pub vao: u32,
    vbo: u32,
}

impl Vao {
    pub fn create(program: &Program, vertices: &Vec<f32>, shader_params: &Option<Vec<ShaderParam>>) -> Self {
        unsafe {
            let gl_program = program.program as GLuint;
            let mut gl_vao = 0;
            let mut gl_vbo = 0;

            gl::UseProgram(gl_program);

            gl::GenVertexArrays(1, &mut gl_vao);
            gl::GenBuffers(1, &mut gl_vbo);

            gl::BindVertexArray(gl_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl_vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                as_glsizeiptr(vertices),
                as_c_void(vertices),
                gl::STATIC_DRAW
            );
            map_params_to_program(gl_program, shader_params);

            Vao{ vao: gl_vao, vbo: gl_vbo }
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, as_const_gluint(self.vao));
            gl::DeleteBuffers(1, as_const_gluint(self.vbo));
        }
    }
}