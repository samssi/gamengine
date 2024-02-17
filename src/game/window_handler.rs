use glfw::{Key, PWindow, Window};
use glfw::ffi::GLFWwindow;
use crate::game::context::GameState;
use crate::state::context::GameContext;

// TODO: need to be mapped on engine side and not game side like process_events
pub fn glfw_window_handler(window: &mut Window, x_pos: f64, y_pos: f64) {
    println!("{}, {}", x_pos, y_pos);
}