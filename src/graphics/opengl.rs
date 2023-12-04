use gl::types::*;
use std::fs;

pub fn gl_init() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // TODO: make globally configurable
        gl::Viewport(0, 0, 800, 600);
    }
}

fn draw_entity() {

}

pub fn gl_render() {
    println!("OpenGL rendering graphics...");
    draw_entity();
}