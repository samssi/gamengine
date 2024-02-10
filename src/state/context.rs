use std::collections::HashMap;
use glfw::{Glfw, PWindow};
use crate::io::keyboard::KeyActivity;

pub struct WindowProperties {
    pub width: u32,
    pub height: u32,
}

pub struct WindowContext {
    pub window: PWindow,
    pub window_properties: WindowProperties,
    pub glfw: Glfw,
    pub keymap: HashMap<String, KeyActivity>
}