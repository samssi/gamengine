use crate::game::context::GameState;
use crate::state::context::GameContext;

pub fn glfw_cursor_handler(game_context: &mut GameContext<GameState>, x_pos: f64, y_pos: f64) {
    println!("{}, {}", x_pos, y_pos);
}