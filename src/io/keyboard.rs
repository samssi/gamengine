use std::collections::HashMap;
use glfw::{Action, Key};
use crate::state::context::WindowManagerContext;

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

fn handle_window_control_keyboard_events(context: &mut WindowManagerContext, key: Key, action: Action) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            _ = &context.window.set_should_close(true);
        },
        _ => {}
    }
}

fn handle_engine_keyboard_press_events(context: &mut WindowManagerContext, key: Key) {
    let key_as_string = key_to_string(key);

    let direction = key_as_string.and_then(|key| context.keymap.get(&key).cloned());
    println!("Direction: {:?} pressed", direction);

    match direction {
        Some(KeyActivity::LEFT) => {
            let x_position = context.entity.transform.position.x;
            context.entity.transform.position.x = x_position - 0.1;
        }
        Some(KeyActivity::RIGHT) => {
            let x_position = context.entity.transform.position.x;
            context.entity.transform.position.x = x_position + 0.1;
        }
        Some(KeyActivity::UP) => {
            let y_position = context.entity.transform.position.y;
            context.entity.transform.position.y = y_position + 0.1;
        }
        Some(KeyActivity::DOWN) => {
            let y_position = context.entity.transform.position.y;
            context.entity.transform.position.y = y_position - 0.1;
        }
        Some(KeyActivity::ROTATE_CLOCKWISE) => {
            let rotation = context.entity.transform.rotation.z;
            context.entity.transform.rotation.z = rotation + 0.1;
        }
        Some(KeyActivity::ROTATE_COUNTER_CLOCKWISE) => {
            let rotation = context.entity.transform.rotation.z;
            context.entity.transform.rotation.z = rotation - 0.1;
        }
        Some(KeyActivity::SCALE_ALL_UP) => {
            let scale_x = context.entity.transform.scale.x;
            let scale_y = context.entity.transform.scale.y;
            let scale_z = context.entity.transform.scale.z;

            context.entity.transform.scale.x = scale_x + 0.1;
            context.entity.transform.scale.y = scale_y + 0.1;
            context.entity.transform.scale.z = scale_z + 0.1;
        }
        Some(KeyActivity::SCALE_ALL_DOWN) => {
            let scale_x = context.entity.transform.scale.x;
            let scale_y = context.entity.transform.scale.y;
            let scale_z = context.entity.transform.scale.z;

            context.entity.transform.scale.x = scale_x - 0.1;
            context.entity.transform.scale.y = scale_y - 0.1;
            context.entity.transform.scale.z = scale_z - 0.1;
        }
        _ => {}
    }
}

pub fn handle_keyboard_events(context: &mut WindowManagerContext, key: Key, action: Action) {
    handle_window_control_keyboard_events(context, key, action);
    match action {
        Action::Press => {
            handle_engine_keyboard_press_events(context, key);
        },
        _ => {}
    }
}
