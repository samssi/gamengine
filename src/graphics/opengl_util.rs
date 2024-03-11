use std::ffi::{c_void, CString};
use std::mem;
use gl::types::{GLenum, GLfloat, GLint, GLsizeiptr, GLuint};

pub enum ShaderType {
    VertexShader,
    FragmentShader
}

pub fn shader_type_to_glenum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => { gl::VERTEX_SHADER }
        ShaderType::FragmentShader => { gl::FRAGMENT_SHADER }
    }
}

pub fn as_c_string(string: &str) -> CString {
    CString::new(string).expect("CString::new failed")
}

pub fn get_uniform_location(program: GLuint, uniform_name: &str) -> GLint {
    let attribute_name_cstring = CString::new(uniform_name).expect("CString conversion failed");
    unsafe {
        gl::GetUniformLocation(program, attribute_name_cstring.as_ptr())
    }
}

pub fn get_attrib_location(program: &GLuint, attribute_name: &str) -> GLuint {
    let attribute_name_cstring = CString::new(attribute_name).expect("CString conversion failed");
    unsafe {
        let location = gl::GetAttribLocation(*program, attribute_name_cstring.as_ptr());
        if location != -1 {
            location as GLuint
        }
        else {
            panic!("Failed to retrieve attrib location")
        }
    }
}

pub fn as_const_gluint(value: u32) -> *const GLuint {
    value as *const GLuint
}

pub fn as_c_void(vertices: &Vec<f32>) -> *const c_void {
    vertices.as_ptr() as *const c_void
}

pub fn as_glsizeiptr(vertices: &Vec<f32>) -> GLsizeiptr {
    (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr
}