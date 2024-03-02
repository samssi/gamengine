use nalgebra::clamp;
use crate::game::context::GameState;
use crate::game::context::Mode::CAMERA;
use crate::state::context::{GameContext, WindowContext, WindowState};

fn limited_delta_time(delta_time: u128) -> f64 {
    let new_delta = delta_time as f64 / 10.0;
    clamp(new_delta, -1.0, 1.0)
}

fn calculate_cursor_acceleration(game_context: &mut GameContext<GameState>, x_pos: f64, y_pos: f64, delta_time: f64) -> (f32, f32) {
    let x_sensitivity = game_context.mouse_context.x_sensitivity;
    let y_sensitivity = game_context.mouse_context.y_sensitivity;
    let window_width = game_context.window_context.window_properties.width as f64;
    let window_height = game_context.window_context.window_properties.height as f64;
    let previous_x_pos = game_context.window_context.window_state.cursor.previous_x_pos;
    let previous_y_pos = game_context.window_context.window_state.cursor.previous_y_pos;

    let acceleration_x = (x_pos - previous_x_pos) / window_width * x_sensitivity * delta_time;
    let acceleration_y = (y_pos - previous_y_pos) / window_height * y_sensitivity * delta_time;
    (acceleration_x as f32, acceleration_y as f32)
}

pub fn glfw_cursor_handler(game_context: &mut GameContext<GameState>, x_pos: f64, y_pos: f64) {
    let delta_time = limited_delta_time(game_context.window_context.window_state.delta_time);

    if game_context.game.state.mode == CAMERA {
        let (acceleration_x, acceleration_y) = calculate_cursor_acceleration(game_context, x_pos, y_pos, delta_time);
        let mut camera_rotation = &mut game_context.entity_context.cameras[0].transform.rotation;
        camera_rotation.z = camera_rotation.z + acceleration_x;
        camera_rotation.y = camera_rotation.y + acceleration_y;
    }

    game_context.window_context.window_state.cursor.previous_x_pos = x_pos;
    game_context.window_context.window_state.cursor.previous_y_pos = y_pos;
}