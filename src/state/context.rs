use std::collections::HashMap;
use glfw::PWindow;
use crate::io::keyboard::Direction;

pub struct WindowManagerContext<'context> {
    pub window: &'context mut PWindow,
    pub keymap: &'context HashMap<&'context str, Direction>
}

impl <'context> WindowManagerContext<'context> {
    pub fn new(window: &'context mut PWindow, keymap: &'context HashMap<&'context str, Direction>) -> Self {
        Self {
            window,
            keymap
        }
    }

}