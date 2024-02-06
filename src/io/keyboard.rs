use std::collections::HashMap;
use glfw::{Action, Key};
use crate::state::context::WindowManagerContext;

#[derive(Debug, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "w" => Some(Key::W),
        "a" => Some(Key::A),
        "s" => Some(Key::S),
        "d" => Some(Key::D),
        _ => None
    }
}

fn key_to_string<'keytostr>(key: Key) -> Option<&'keytostr str> {
    match key {
        Key::W => Some("w"),
        Key::A => Some("a"),
        Key::S => Some("s"),
        Key::D => Some("d"),
        _ => None
    }
}

pub fn create_keymap<'dirmap>() -> HashMap<&'dirmap str, Direction> {
    let mut map = HashMap::new();
    map.insert("w", Direction::UP);
    map.insert("s", Direction::DOWN);
    map.insert("a", Direction::LEFT);
    map.insert("d", Direction::RIGHT);
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
        Some(Direction::LEFT) => {
            let x_position = context.entity.transform.position.x;
            context.entity.transform.position.x = x_position - 0.1;
        }
        Some(Direction::RIGHT) => {
            let x_position = context.entity.transform.position.x;
            context.entity.transform.position.x = x_position + 0.1;
        }
        Some(Direction::UP) => {
            let y_position = context.entity.transform.position.y;
            context.entity.transform.position.y = y_position + 0.1;
        }
        Some(Direction::DOWN) => {
            let y_position = context.entity.transform.position.y;
            context.entity.transform.position.y = y_position - 0.1;
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
