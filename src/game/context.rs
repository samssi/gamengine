use glfw::CursorMode::{Disabled, Normal};
use crate::state::context::GameContext;

pub fn toggle_cursor_mode(game_context: &mut GameContext<GameState>) {
    game_context.game.state.lock_to_window = !game_context.game.state.lock_to_window;
    if game_context.game.state.lock_to_window {
        game_context.window_context.window.set_cursor_mode(Disabled);
    }
    else { game_context.window_context.window.set_cursor_mode(Normal); }
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    CAMERA,
    OBJECT
}
pub struct GameState {
    pub mode: Mode,
    pub lock_to_window: bool
}