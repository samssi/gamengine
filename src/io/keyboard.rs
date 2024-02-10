use std::collections::HashMap;
use glfw::{Action, Key};
use crate::entity::entity::Entity3d;
use crate::state::context::WindowContext;
use crate::state::entity_context::EntityContext;

#[derive(Debug, Clone)]
pub enum KeyActivity {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ROTATE_CLOCKWISE,
    ROTATE_COUNTER_CLOCKWISE,
    SCALE_ALL_UP,
    SCALE_ALL_DOWN
}

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "w" => Some(Key::W),
        "a" => Some(Key::A),
        "s" => Some(Key::S),
        "d" => Some(Key::D),
        "q" => Some(Key::Q),
        "e" => Some(Key::E),
        "keypad-add" => Some(Key::KpAdd),
        "keypad-subtract" => Some(Key::KpSubtract),
        _ => None
    }
}

fn key_to_string<'keytostr>(key: Key) -> Option<&'keytostr str> {
    match key {
        Key::W => Some("w"),
        Key::A => Some("a"),
        Key::S => Some("s"),
        Key::D => Some("d"),
        Key::Q => Some("q"),
        Key::E => Some("e"),
        Key::KpAdd => Some("keypad-add"),
        Key::KpSubtract=> Some("keypad-subtract"),
        _ => None
    }
}

pub fn create_keymap<'dirmap>() -> HashMap<&'dirmap str, KeyActivity> {
    // TODO: use HashMap::from
    let mut map = HashMap::new();
    map.insert("w", KeyActivity::UP);
    map.insert("s", KeyActivity::DOWN);
    map.insert("a", KeyActivity::LEFT);
    map.insert("d", KeyActivity::RIGHT);
    map.insert("e", KeyActivity::ROTATE_CLOCKWISE);
    map.insert("q", KeyActivity::ROTATE_COUNTER_CLOCKWISE);
    map.insert("keypad-add", KeyActivity::SCALE_ALL_UP);
    map.insert("keypad-subtract", KeyActivity::SCALE_ALL_DOWN);
    map
}

fn handle_window_control_keyboard_events(context: &mut WindowContext, key: Key, action: Action) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            _ = &context.window.set_should_close(true);
        },
        _ => {}
    }
}

fn handle_entity_keyboard_press_events(window_context: &mut WindowContext, entity: &mut Entity3d, key: Key) {
    let key_as_string = key_to_string(key);

    let direction = key_as_string.and_then(|key| window_context.keymap.get(&key).cloned());
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

pub fn handle_keyboard_events(window_context: &mut WindowContext, entity: &mut Entity3d, key: Key, action: Action) {
    handle_window_control_keyboard_events(window_context, key, action);
    match action {
        Action::Press => {
            handle_entity_keyboard_press_events(window_context, entity, key);
        },
        _ => {}
    }
}
