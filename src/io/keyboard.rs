use std::collections::HashMap;
use glfw::{Action, Key};
use crate::state::context::{GameContext};

#[derive(Clone)]
pub enum KeyActivity {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ROTATE_CLOCKWISE,
    ROTATE_COUNTER_CLOCKWISE,
    SCALE_ALL_UP,
    SCALE_ALL_DOWN,
    MODE,
    LOCK_MOUSE_INTO_VIEW
}

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "w" => Some(Key::W),
        "a" => Some(Key::A),
        "s" => Some(Key::S),
        "d" => Some(Key::D),
        "q" => Some(Key::Q),
        "e" => Some(Key::E),
        "tab" => Some(Key::Tab),
        "space" => Some(Key::Space),
        "keypad-add" => Some(Key::KpAdd),
        "keypad-subtract" => Some(Key::KpSubtract),
        _ => None
    }
}

pub fn key_to_string(key: Key) -> Option<String> {
    match key {
        Key::W => Some(String::from("w")),
        Key::A => Some(String::from("a")),
        Key::S => Some(String::from("s")),
        Key::D => Some(String::from("d")),
        Key::Q => Some(String::from("q")),
        Key::E => Some(String::from("e")),
        Key::Space => Some(String::from("space")),
        Key::KpAdd => Some(String::from("keypad-add")),
        Key::KpSubtract=> Some(String::from("keypad-subtract")),
        Key::Tab => Some(String::from("tab")),
        _ => None
    }
}

pub fn create_keymap() -> HashMap<String, KeyActivity> {
    HashMap::from([
        (String::from("w"), KeyActivity::UP),
        (String::from("s"), KeyActivity::DOWN),
        (String::from("a"), KeyActivity::LEFT),
        (String::from("d"), KeyActivity::RIGHT),
        (String::from("e"), KeyActivity::ROTATE_CLOCKWISE),
        (String::from("q"), KeyActivity::ROTATE_COUNTER_CLOCKWISE),
        (String::from("keypad-add"), KeyActivity::SCALE_ALL_UP),
        (String::from("keypad-subtract"), KeyActivity::SCALE_ALL_DOWN),
        (String::from("space"), KeyActivity::MODE),
        (String::from("tab"), KeyActivity::LOCK_MOUSE_INTO_VIEW),
    ])
}

fn handle_window_control_keyboard_events<T>(context: &mut GameContext<T>, key: Key, action: Action) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            _ = &mut context.window_context.window.set_should_close(true);
        },
        _ => {}
    }
}

pub fn handle_keyboard_events<T>(game_context: &mut GameContext<T>,
                              key: Key,
                              action: Action,
                              glfw_press_handler: fn(game_context: &mut GameContext<T>, key: Key)) {
    handle_window_control_keyboard_events(game_context, key, action);
    match action {
        Action::Press => {
            glfw_press_handler(game_context, key);
        },
        _ => {}
    }
}
