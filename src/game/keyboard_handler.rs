use glfw::CursorMode::{Disabled, Normal};
use glfw::Key;
use crate::game::context::{GameState, toggle_cursor_mode};
use crate::game::context::Mode::{CAMERA, OBJECT};
use crate::io::keyboard::{key_to_string, KeyActivity};
use crate::state::context::{Game, GameContext};

pub fn window_lock(game_context: &mut GameContext<GameState>, key_activity: &Option<KeyActivity>) {
    match key_activity {
        Some(KeyActivity::LOCK_MOUSE_INTO_VIEW) => {
            toggle_cursor_mode(game_context);
        }
        _ => {}
    }
}

fn camera_mode(game_context: &mut GameContext<GameState>, key_activity: &Option<KeyActivity>) {
    let camera = &mut game_context.entity_context.cameras[0];

    match key_activity {
        Some(KeyActivity::LEFT) => {
            let mut x_position = camera.transform.position.x;
            camera.transform.position.x = x_position - 10.0;
        }
        Some(KeyActivity::RIGHT) => {
            let mut x_position = camera.transform.position.x;
            camera.transform.position.x = x_position + 10.0;

        }
        Some(KeyActivity::UP) => {
            let mut z_position = camera.transform.position.z;
            camera.transform.position.z = z_position + 10.0;
        }
        Some(KeyActivity::DOWN) => {
            let mut z_position = camera.transform.position.z;
            camera.transform.position.z = z_position - 10.0;
        },
        _ => {}
    }
}

pub fn object_mode(game_context: &mut GameContext<GameState>, key_activity: &Option<KeyActivity>) {
    let entity = &mut game_context.entity_context.entities[0];

    match key_activity {
        Some(KeyActivity::LEFT) => {
            let x_position = entity.transform.position.x;
            entity.transform.position.x = x_position - 10.0;
        }
        Some(KeyActivity::RIGHT) => {
            let x_position = entity.transform.position.x;
            entity.transform.position.x = x_position + 10.0;
        }
        Some(KeyActivity::UP) => {
            let y_position = entity.transform.position.y;
            entity.transform.position.y = y_position + 10.0;
        }
        Some(KeyActivity::DOWN) => {
            let y_position = entity.transform.position.y;
            entity.transform.position.y = y_position - 10.0;
        }
        Some(KeyActivity::ROTATE_CLOCKWISE) => {
            let rotation = entity.transform.rotation.z;
            entity.transform.rotation.z = rotation + 10.0;
        }
        Some(KeyActivity::ROTATE_COUNTER_CLOCKWISE) => {
            let rotation = entity.transform.rotation.z;
            entity.transform.rotation.z = rotation - 10.0;
        }
        Some(KeyActivity::SCALE_ALL_UP) => {
            let scale_x = entity.transform.scale.x;
            let scale_y = entity.transform.scale.y;
            let scale_z = entity.transform.scale.z;

            entity.transform.scale.x = scale_x + 0.1;
            entity.transform.scale.y = scale_y + 0.1;
            entity.transform.scale.z = scale_z + 0.1;
        }
        Some(KeyActivity::SCALE_ALL_DOWN) => {
            let scale_x = entity.transform.scale.x;
            let scale_y = entity.transform.scale.y;
            let scale_z = entity.transform.scale.z;

            entity.transform.scale.x = scale_x - 0.1;
            entity.transform.scale.y = scale_y - 0.1;
            entity.transform.scale.z = scale_z - 0.1;
        }
        _ => {}
    };
}

fn handle_events(game_context: &mut GameContext<GameState>, key_activity: &Option<KeyActivity>) {
    match key_activity {
        Some(KeyActivity::MODE) => {
            let mut mode = &mut game_context.game.state.mode;
            match mode {
                CAMERA => {
                    game_context.game.state.mode = OBJECT;
                }
                OBJECT => {
                    game_context.game.state.mode = CAMERA;
                }
            }
        }
        _ => {}
    }

    match game_context.game.state.mode {
        CAMERA => {
            camera_mode(game_context, &key_activity)
        }
        OBJECT => {
            object_mode(game_context, &key_activity)
        }
    }
}

pub fn glfw_press_handler(game_context: &mut GameContext<GameState>, key: Key) {
    let key_as_string = key_to_string(key);
    let key_activity = key_as_string
        .and_then(|key| {
            let keymap = &mut game_context.keyboard_context.keymap;
                keymap.get(&key).cloned()
        });

    if game_context.game.state.lock_to_window {
        handle_events(game_context, &key_activity)
    }
    window_lock(game_context, &key_activity);
}