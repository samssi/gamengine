mod state;
mod graphics;
mod os;
mod entity;
mod io;
mod game;

use crate::game::game::start;
use crate::os::window_manager::start_opengl_window_manager;

fn main() {
    start()
}