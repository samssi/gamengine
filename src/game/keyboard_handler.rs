use glfw::Key;
use crate::io::keyboard::{key_to_string, KeyActivity};
use crate::state::context::GameContext;

pub fn glfw_press_handler(game_context: &mut GameContext, key: Key) {
    let entity = &mut game_context.entity_context.entities[0];
    let key_as_string = key_to_string(key);

    let direction = key_as_string.and_then(|key| game_context.keyboard_context.keymap.get(&key));
    println!("Direction: {:?} pressed", direction);

    match direction {
        Some(KeyActivity::LEFT) => {
            let x_position = entity.transform.position.x;
            entity.transform.position.x = x_position - 0.1;
        }
        Some(KeyActivity::RIGHT) => {
            let x_position = entity.transform.position.x;
            entity.transform.position.x = x_position + 0.1;
        }
        Some(KeyActivity::UP) => {
            let y_position = entity.transform.position.y;
            entity.transform.position.y = y_position + 0.1;
        }
        Some(KeyActivity::DOWN) => {
            let y_position = entity.transform.position.y;
            entity.transform.position.y = y_position - 0.1;
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
    }
}