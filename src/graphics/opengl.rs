use std::ffi::{c_void, CString};
use gl::types::*;
use std::{mem, ptr};
use crate::entity::entity::Entity3d;
use crate::graphics::calculations::apply_3d_transformations;

// TODO: make globally configurable
const HEIGHT: i32 = 800;
const WIDTH: i32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

fn get_attrib_location(program: GLuint, attribute_name: &str) -> GLint {
    let attribute_name_cstring = CString::new(attribute_name).expect("CString conversion failed");
    unsafe {
        gl::GetAttribLocation(program, attribute_name_cstring.as_ptr())
    }
}

const VERTEX_SHADER_SOURCE_2: &str = r#"
    #version 330 core
    in vec4 a_position;
    uniform mat4 u_matrix;

    void main() {
        gl_Position = u_matrix * a_position;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;

pub fn gl_init() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::Viewport(0, 0, WIDTH, HEIGHT);
    }
}

fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, String> {
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

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) ->  Result<GLuint, String> {
    let program;
    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);

        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

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

fn create_vao(vertices: &Vec<f32>) -> GLuint {
    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        /*
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);*/
    }
    vao
}

fn set_geometry(entity: &Entity3d, program: GLuint) {
    unsafe {
        let vao = create_vao(&entity.points);

    }
}

fn draw_entity(entity_3d: &Entity3d) {
    unsafe {
        let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE_2, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
        let program = link_program(vertex_shader.unwrap(), fragment_shader.unwrap()).unwrap();

        gl::Enable(gl::CULL_FACE);
        // gl::Enable(gl::DEPTH_TEST);
        gl::UseProgram(program);

        //let vao = create_vao(TRIANGLE);
        let vao = create_vao(&entity_3d.points);

        // TODO: calculations
        let final_matrix = apply_3d_transformations(&entity_3d);

        // gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix_data.as_ptr());
        let matrix_data: Vec<f32> = final_matrix.as_slice().to_owned();
        // let count = matrix_data.len() as GLsizei;

        gl::UniformMatrix4fv(get_attrib_location(program, "u_matrix"), 1, gl::FALSE, matrix_data.as_ptr());

        gl::BindVertexArray(vao);

        gl::DrawArrays(gl::TRIANGLES, 0, 3);

    }
}


#[allow(dead_code)]
fn print_fps(delta_time: u128) {
    if delta_time > 0 {
        println!("fps: {:?}", 1000 / delta_time);
    }
}

const TRIANGLE: [f32; 9] =
    [  -1.0, -0.5, 0.0,
        1.0, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

pub fn gl_render(_delta_time: u128) {
    //print_fps(delta_time);
    let entity_3d = Entity3d::new(TRIANGLE.to_vec());
    draw_entity(&entity_3d);
}