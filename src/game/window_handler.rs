use glfw::{Key, PWindow, Window};
use glfw::ffi::GLFWwindow;
use crate::game::context::GameState;
use crate::state::context::GameContext;

pub fn glfw_cursor_handler<T>(game_context: &mut GameContext<T>, x_pos: f64, y_pos: f64) {
    println!("{}, {}", x_pos, y_pos);
}