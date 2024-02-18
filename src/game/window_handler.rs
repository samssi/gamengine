use nalgebra::clamp;
use crate::game::context::GameState;
use crate::state::context::{GameContext, WindowContext, WindowState};

fn limited_delta_time(delta_time: u128) -> f64 {
    let new_delta = delta_time as f64 / 10.0;
    clamp(new_delta, -1.0, 1.0)
}

fn calculate_cursor_acceleration(game_context: &mut GameContext<GameState>, x_pos: f64, y_pos: f64, delta_time: f64) -> (f64, f64) {
    let sensitivity = game_context.mouse_context.sensitivity;
    let window_width = game_context.window_context.window_properties.width as f64;
    let window_height = game_context.window_context.window_properties.height as f64;
    let previous_x_pos = game_context.window_context.window_state.cursor.previous_x_pos;
    let previous_y_pos = game_context.window_context.window_state.cursor.previous_y_pos;

    let acceleration_x = (x_pos - previous_x_pos) / window_width * sensitivity * delta_time;
    let acceleration_y = (y_pos - previous_y_pos) / window_height * sensitivity * delta_time;
    (acceleration_x, acceleration_y)
}

pub fn glfw_cursor_handler(game_context: &mut GameContext<GameState>, x_pos: f64, y_pos: f64) {
    let delta_time = limited_delta_time(game_context.window_context.window_state.delta_time);

    if game_context.game.state.camera_mode {
        let (x_acceleration, y_acceleration) = calculate_cursor_acceleration(game_context, x_pos, y_pos, delta_time);
        println!("delta: {}, x acceleration {}, y acceleration {}", delta_time, x_acceleration, y_acceleration);
        // println!("{}", diff_delta_time);
        // println!("{}", acceleration);
    }

    game_context.window_context.window_state.cursor.previous_x_pos = x_pos;
    game_context.window_context.window_state.cursor.previous_y_pos = y_pos;
}