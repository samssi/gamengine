use gl::types::{GLenum, GLuint};
use crate::graphics::opengl_util::{as_c_string, as_c_void, as_const_gluint, as_glsizeiptr, ShaderType};

struct Shader {
    shader_type: ShaderType,
    shader_source_code: String
}

impl Shader {
    fn create(shader_type: ShaderType, shader_source_code: String) -> Self {
        unsafe {
            let gl_shader_source = as_c_string(&shader_source_code);

            Shader{shader_type, shader_source_code}
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        todo!()
    }
}

struct Vao {
    vao: u32,
    vbo: u32
}

impl Vao {
    fn create(program: u32, vertices: &Vec<f32>) -> Self {
        unsafe {
            let gl_program = program as GLuint;
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