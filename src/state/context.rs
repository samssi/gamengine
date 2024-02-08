use std::collections::HashMap;
use gl::types::GLuint;
use glfw::PWindow;
use crate::entity::entity::{Entity3d, TRIANGLE};
use crate::io::keyboard::KeyActivity;

pub struct WindowManagerContext<'context> {
    pub window: &'context mut PWindow,
    pub keymap: &'context HashMap<&'context str, KeyActivity>,
    // TODO: these below shouldn't probably be in this context
    pub entity: &'context mut Entity3d,
    pub vertex_shaders: &'context HashMap<String, GLuint>,
    pub fragment_shaders: &'context HashMap<String, GLuint>,
}

impl <'context> WindowManagerContext<'context> {
    pub fn new(
        window: &'context mut PWindow,
        keymap: &'context HashMap<&'context str, KeyActivity>,
        entity: &'context mut Entity3d,
        vertex_shaders: &'context HashMap<String, GLuint>,
        fragment_shaders: &'context HashMap<String, GLuint>
    ) -> Self {
        Self {
            window,
            keymap,
            entity,
            vertex_shaders,
            fragment_shaders
        }
    }

}