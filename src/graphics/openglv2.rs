use std::any::Any;
use std::ffi::c_void;
use std::ptr;
use gl::types::{GLenum, GLint, GLsizei, GLuint};
use image::{DynamicImage, EncodableLayout};
use crate::graphics::opengl_util::{as_c_string, as_c_void, as_const_gluint, as_glsizeiptr, as_stride, get_attrib_location, get_program_compilation_status, get_shader_compilation_status, image_as_c_void, map_params_to_program};

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

pub struct VertexShader {
    pub shader: u32,
    pub shader_source_code: String,
    pub shader_params: Option<Vec<ShaderParam>>
}

impl VertexShader {
    pub fn create(shader_source_code: String, shader_params: Option<Vec<ShaderParam>>) -> Self {
        unsafe {
            let gl_shader_source = as_c_string(&shader_source_code);
            let gl_shader = gl::CreateShader(gl::VERTEX_SHADER);

            gl::ShaderSource(gl_shader, 1, &gl_shader_source.as_ptr(), ptr::null());
            gl::CompileShader(gl_shader);
            get_shader_compilation_status(gl_shader).expect("Vertex shader compilation failure!");

            Self {shader: gl_shader, shader_source_code, shader_params}
        }
    }
}

pub struct FragmentShader {
    pub shader: u32,
    pub shader_source_code: String,
}

impl FragmentShader {
    pub fn create(shader_source_code: String) -> Self {
        unsafe {
            let gl_shader_source = as_c_string(&shader_source_code);
            let gl_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            gl::ShaderSource(gl_shader, 1, &gl_shader_source.as_ptr(), ptr::null());
            gl::CompileShader(gl_shader);
            get_shader_compilation_status(gl_shader).expect("Fragment shader compilation failure!");

            Self {shader: gl_shader, shader_source_code}
        }
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.shader as GLuint) }
    }
}

pub struct Program {
    pub program: u32
}

impl Program {
    pub fn create(vertex_shader: &VertexShader, fragment_shader: &FragmentShader) -> Self {
        unsafe {
            let gl_program = gl::CreateProgram();
            let gl_vertex_shader: GLuint = vertex_shader.shader as GLuint;
            let gl_fragment_shader: GLuint = fragment_shader.shader as GLuint;

            gl::AttachShader(gl_program, gl_vertex_shader);
            gl::AttachShader(gl_program, gl_fragment_shader);

            gl::LinkProgram(gl_program);
            get_program_compilation_status(gl_program).expect("Shader linking failure while creating program!");

            Self{program: gl_program}
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program as GLuint) }
    }
}

pub struct Texture {
    texture: u32,
    texture_coordinate_buffer: u32,
    image: DynamicImage
}

impl Texture {
    pub fn create(texture_image: DynamicImage, texture_coordinates: Vec<f32>) -> Self {
        unsafe {
            let image_rgba8 = texture_image.to_rgba8();
            let mut gl_texture = 0;
            let mut gl_texture_coordinate_buffer = 0;

            gl::GenBuffers(1, &mut gl_texture_coordinate_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl_texture_coordinate_buffer);

            gl::GenTextures(1, &mut gl_texture);
            gl::BindTexture(gl::TEXTURE_2D, gl_texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLsizei,
                image_rgba8.width() as GLsizei,
                image_rgba8.height() as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_as_c_void(image_rgba8)
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                as_glsizeiptr(&texture_coordinates),
                texture_coordinates.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );

            /*
            TODO: vec2 mapping action and Vao passing action
            gl::VertexAttribPointer(
            a_texture_coordinates,
            2,
            gl::FLOAT,
            gl::FALSE,
            as_stride(2),
            ptr::null());

            gl::EnableVertexAttribArray(a_texture_coordinates);
            gl::BindVertexArray(vao);
             */

            Self {
                image: texture_image,
                texture: gl_texture,
                texture_coordinate_buffer: gl_texture_coordinate_buffer
            }
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, as_const_gluint(self.texture));
            gl::DeleteBuffers(1, as_const_gluint(self.texture_coordinate_buffer));
        }
    }
}

pub struct Vao {
    pub vao: u32,
    vbo: u32,
}

impl Vao {
    pub fn create(program: &Program, vertices: &Vec<f32>, vertex_shader: &VertexShader) -> Self {
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
            map_params_to_program(gl_program, &vertex_shader.shader_params);

            Self{ vao: gl_vao, vbo: gl_vbo }
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