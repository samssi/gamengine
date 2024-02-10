use std::collections::HashMap;
use glfw::{Glfw, PWindow};
use gl::types::GLuint;
use crate::entity::entity::Entity3d;
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

pub struct ShaderContext {
    pub vertex_shaders: HashMap<String, GLuint>,
    pub fragment_shaders: HashMap<String, GLuint>
}

pub struct EntityContext {
    pub entities: Vec<Entity3d>,
}

pub struct GameContext {
    pub window_context: WindowContext,
    pub shader_context: ShaderContext,
    pub entity_context: EntityContext
}