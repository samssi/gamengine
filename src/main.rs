mod state;
mod graphics;
mod os;
mod entity;
mod io;

use crate::os::window_manager::start_window_manager;

fn main() {
    start_window_manager();
}