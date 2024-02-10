use std::collections::HashMap;
use gl::types::GLuint;
use glfw::PWindow;
use crate::entity::entity::{Entity3d, TRIANGLE};
use crate::io::keyboard::KeyActivity;

pub struct WindowProperties {
    pub width: u32,
    pub height: u32,
}

pub struct WindowContext<'context> {
    pub window: &'context mut PWindow,
    pub window_properties: &'context WindowProperties,
    pub keymap: &'context HashMap<&'context str, KeyActivity>,
}

impl <'context> WindowContext<'context> {
    pub fn new(
        window: &'context mut PWindow,
        window_properties: &'context WindowProperties,
        keymap: &'context HashMap<&'context str, KeyActivity>
    ) -> Self {
        Self {
            window,
            window_properties,
            keymap
        }
    }
}