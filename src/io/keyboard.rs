use glfw::{Action, Key, PWindow};
use crate::io::keyboard::Direction::{E, N, S, W};

fn handle_window_control_keyboard_events(window: &mut PWindow, key: Key, action: Action) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            window.set_should_close(true);
        },
        _ => {}
    }
}

#[derive(Debug)]
pub enum Direction {
    N,
    S,
    E,
    W
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

fn handle_engine_keyboard_press_events(key: Key) -> Option<Direction> {
    match key {
        (Key::W) => {
            return Some(N);
        },
        (Key::S) => {
            return Some(S);
        },
        (Key::D) => {
            return Some(E);
        },
        (Key::A) => {
            return Some(W);
        },
        _ => None
    }
}

pub fn handle_keyboard_events(window: &mut PWindow, key: Key, action: Action) {
    handle_window_control_keyboard_events(window, key, action);
    match action {
        (Action::Press) => {
            println!("Direction: {:?} pressed", handle_engine_keyboard_press_events(key))
        },
        _ => {}
    }
}