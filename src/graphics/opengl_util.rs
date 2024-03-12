use std::ffi::{c_void, CString};
use std::{mem, ptr};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizeiptr, GLuint};

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

pub fn as_const_glchar(value: u32) -> *const GLuint {
    value as *const GLuint
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

pub fn get_compilation_status(gl_id: GLuint) -> Result<GLuint, String> {
    unsafe {
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &mut success);

        if success == gl::TRUE as GLint {
            Ok(gl_id)
        }
        else {
            let mut len: GLint = 0;
            gl::GetShaderiv(gl_id, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize));

            gl::GetShaderInfoLog(gl_id, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);

            Err(String::from_utf8_lossy(&buffer).into_owned())
        }
    }
}