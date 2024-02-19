use std::ffi::{c_void, CString};
use gl::types::*;
use std::{mem, ptr};
use std::collections::HashMap;
use crate::entity::camera::Camera;
use crate::entity::entity::{Entity3d};
use crate::graphics::calculations::apply_3d_transformations_perspective;
use crate::state::context::{ShaderContext, WindowContext};
use crate::state::context::EntityContext;


fn get_attrib_location(program: &GLuint, attribute_name: &str) -> GLuint {
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

fn get_uniform_location(program: GLuint, uniform_name: &str) -> GLint {
    let attribute_name_cstring = CString::new(uniform_name).expect("CString conversion failed");
    unsafe {
        gl::GetUniformLocation(program, attribute_name_cstring.as_ptr())
    }
}

fn create_shader(source: &str, shader_type: GLenum) -> Result<GLuint, String> {
    let c_str_source = CString::new(source).expect("CString::new failed");
    let shader;
    unsafe {
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader,1, &c_str_source.as_ptr(), ptr::null());
        gl::CompileShader(shader)
    }

    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == gl::TRUE as GLint {
        Ok(shader)
    }
    else {
        // Retrieve and print the compilation error log
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
        buffer.extend([b' '].iter().cycle().take(len as usize));

        unsafe {
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
        }

        Err(String::from_utf8_lossy(&buffer).into_owned())
    }
}

pub fn link_shaders(vertex_shader: &GLuint, fragment_shader: &GLuint) ->  Result<GLuint, String> {
    let program;
    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, *vertex_shader);
        gl::AttachShader(program, *fragment_shader);

        gl::LinkProgram(program);

        gl::DeleteShader(*vertex_shader);
        gl::DeleteShader(*fragment_shader);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == gl::TRUE as GLint {
            Ok(program)
        } else {
            // Retrieve and print the linking error log
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            Err(String::from_utf8_lossy(&buffer).into_owned())
        }
    }
}

pub fn create_program(vertex_shader: &GLuint, fragment_shader: &GLuint) -> GLuint {
    link_shaders(&vertex_shader, &fragment_shader)
        .expect(&format!("shader linking failed for vertex shader {} and fragment shader {}",
                         vertex_shader, fragment_shader))
}

pub fn create_vao(program: &GLuint, vertices: &Vec<f32>) -> GLuint {
    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        gl::Enable(gl::CULL_FACE);
        //gl::Enable(gl::DEPTH_TEST);
        gl::UseProgram(*program);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        let attrib_location = get_attrib_location(program, "a_position");
        gl::EnableVertexAttribArray(attrib_location);
        gl::VertexAttribPointer(attrib_location,
                                3,                              // size (number of components)
                                gl::FLOAT,                      // type
                                gl::FALSE,                      // normalized
                                (3 * mem::size_of::<GLfloat>()) as GLsizei, // stride (byte offset between consecutive generic vertex attributes)
                                ptr::null());



        // Unbind VBO and VAO
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    vao
}

pub fn create_shader_programs(shaders: HashMap<String, String>, shader_type: GLenum) -> HashMap<String, GLuint> {
    let shader_program_map = shaders
        .keys()
        .fold(HashMap::new(), |mut acc: HashMap<String, GLuint>, key| {
            let fragment_shader_source = shaders.get(key).expect("shader not found");
            let fragment_shader_program = create_shader(fragment_shader_source, shader_type)
                .expect("shader load error");

            acc.entry(key.clone()).or_insert(fragment_shader_program);
            acc
    });

    shader_program_map
}

fn draw_entity(entity_3d: &Entity3d, camera: &Camera) {
    unsafe {
        let final_matrix = apply_3d_transformations_perspective(&entity_3d, camera);
        let matrix_data = final_matrix.as_slice();
        let matrix_data_ptr: *const GLfloat = matrix_data.as_ptr();

        gl::UniformMatrix4fv(get_uniform_location(entity_3d.program, "u_matrix"), 1, gl::FALSE, matrix_data_ptr);
        gl::BindVertexArray(entity_3d.vao);

        let points_len: GLsizei = entity_3d.points.len() as GLsizei;
        gl::DrawArrays(gl::TRIANGLES, 0, points_len);
    }
}

fn draw_entities(context: &EntityContext) {
    context.entities
        .iter()
        .for_each(|entity| draw_entity(entity, &context.cameras[0]));
}


#[allow(dead_code)]
fn print_fps(delta_time: u128) {
    if delta_time > 0 {
        println!("fps: {:?}", 1000 / delta_time);
    }
}

pub fn init_renderer(window_context: &mut WindowContext) {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::Viewport(0, 0, window_context.window_properties.width as GLsizei, window_context.window_properties.height as GLsizei);
    }
}


pub fn render(context: &EntityContext) {
    //print_fps(delta_time);
    draw_entities(context);
}