use std::collections::HashMap;
use glfw::{Action, Key, PWindow};
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

fn key_to_string(key: Key) -> Option<String> {
    match key {
        (Key::W) => Some(String::from("w")),
        (Key::A) => Some(String::from("a")),
        (Key::S) => Some(String::from("s")),
        (Key::D) => Some(String::from("d")),
        _ => None
    }
}


fn create_keymap() -> HashMap<String, Direction> {
    let mut map = HashMap::new();
    map.insert(String::from("w"), Direction::UP);
    map.insert(String::from("s"), Direction::DOWN);
    map.insert(String::from("a"), Direction::LEFT);
    map.insert(String::from("d"), Direction::RIGHT);
    map
}

fn handle_window_control_keyboard_events(context: &mut WindowManagerContext, key: Key, action: Action) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            &context.window.set_should_close(true);
        },
        _ => {}
    }
}

fn handle_engine_keyboard_press_events(key: Key) -> Option<Direction> {
    // TODO: temp, create a context
    let keymap = create_keymap();
    let key_as_string = key_to_string(key);

    key_as_string.and_then(|key| keymap.get(&key).cloned())
}

pub fn handle_keyboard_events(context: &mut WindowManagerContext, key: Key, action: Action) {
    handle_window_control_keyboard_events(context, key, action);
    match action {
        (Action::Press) => {
            println!("Direction: {:?} pressed", handle_engine_keyboard_press_events(key))
        },
        _ => {}
    }
}
