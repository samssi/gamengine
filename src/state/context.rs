use std::collections::HashMap;
use glfw::PWindow;
use crate::entity::entity::{Entity3d, TRIANGLE};
use crate::io::keyboard::KeyActivity;

pub struct WindowManagerContext<'context> {
    pub window: &'context mut PWindow,
    pub keymap: &'context HashMap<&'context str, KeyActivity>,
    pub entity: &'context mut Entity3d
}

impl <'context> WindowManagerContext<'context> {
    pub fn new(
        window: &'context mut PWindow,
        keymap: &'context HashMap<&'context str, KeyActivity>,
        entity: &'context mut Entity3d
    ) -> Self {
        Self {
            window,
            keymap,
            entity
        }
    }

}