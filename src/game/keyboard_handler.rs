use glfw::Key;
use crate::entity::entity::Entity3d;
use crate::game::context::{GameState, Mode};
use crate::game::context::Mode::{CAMERA, OBJECT};
use crate::io::keyboard::{key_to_string, KeyActivity};
use crate::state::context::{Game, GameContext};

fn camera_mode(game_context: &mut GameContext<GameState>, key_activity: Option<KeyActivity>) {
    let camera = &mut game_context.entity_context.cameras[0];

    match key_activity {
        Some(KeyActivity::LEFT) => {
            let x_position = camera.target.x;
            camera.target.x = x_position + 100.0;
        }
        Some(KeyActivity::RIGHT) => {
            let x_position = camera.target.x;
            camera.target.x = x_position - 100.0;
        }
        Some(KeyActivity::UP) => {
            let z_position = camera.transform.position.z;
            camera.transform.position.z = z_position - 100.0;
        }
        Some(KeyActivity::DOWN) => {
            let z_position = camera.transform.position.z;
            camera.transform.position.z = z_position + 100.0;
        }
        _ => {}
    }
}

pub fn object_mode(game_context: &mut GameContext<GameState>, key_activity: Option<KeyActivity>) {
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
            entity.transform.rotation.z = rotation + 0.1;
        }
        Some(KeyActivity::ROTATE_COUNTER_CLOCKWISE) => {
            let rotation = entity.transform.rotation.z;
            entity.transform.rotation.z = rotation - 0.1;
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

pub fn glfw_press_handler(game_context: &mut GameContext<GameState>, key: Key) {
    let key_as_string = key_to_string(key);
    let key_activity = key_as_string
        .and_then(|key| {
            let keymap = &mut game_context.keyboard_context.keymap;
                keymap.get(&key).cloned()
        });

    println!("{:?}", game_context.game.state.mode);
    println!("{:?}", game_context.entity_context.cameras[0]);

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

    let mut mode = &mut game_context.game.state.mode;
    match mode {
        CAMERA => {
            camera_mode(game_context, key_activity)
        }
        OBJECT => {
            object_mode(game_context, key_activity)
        }
    }

    // println!("Direction: {:?} pressed", key_activity);
}