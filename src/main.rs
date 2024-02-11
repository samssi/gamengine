mod state;
mod graphics;
mod os;
mod entity;
mod io;
mod game;

use crate::game::game::start;
use crate::io::loader::read_object_files_into_memory;
use crate::io::object::{parse_points_from_object_file, wavefront_object_as_triangle_points};
use crate::os::window_manager::start_opengl_window_manager;

fn main() {
    //start()
    let files = read_object_files_into_memory();
    let file = files.get("cube.obj").unwrap();
    let wfo = parse_points_from_object_file(file);
    println!("{:?}", wavefront_object_as_triangle_points(wfo));
}