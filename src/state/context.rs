use std::collections::HashMap;
use glfw::{Glfw, PWindow};
use gl::types::GLuint;
use crate::entity::camera::Camera;
use crate::entity::entity::Entity3d;
use crate::io::keyboard::KeyActivity;

pub struct WindowProperties {
    pub width: u32,
    pub height: u32,
}

pub struct WindowContext {
    pub window: PWindow,
    pub window_properties: WindowProperties,
    pub glfw: Glfw
}

pub struct ShaderContext {
    pub vertex_shaders: HashMap<String, GLuint>,
    pub fragment_shaders: HashMap<String, GLuint>,
    pub programs: HashMap<String, GLuint>
}

impl ShaderContext {
    pub fn get_program_or_fail(&self, program: &str) -> GLuint {
        self.programs.get(program).unwrap().clone()
    }
}

pub struct ObjectContext {
    pub objects: HashMap<String, String>
}

pub struct EntityContext {
    pub camera: Vec<Camera>,
    pub entities: Vec<Entity3d>,
}

pub struct KeyboardContext {
    pub keymap: HashMap<String, KeyActivity>
}

pub struct Game<T> {
    pub state: T
}

pub struct GameContext<T> {
    pub window_context: WindowContext,
    pub shader_context: ShaderContext,
    pub object_context: ObjectContext,
    pub entity_context: EntityContext,
    pub keyboard_context: KeyboardContext,
    pub game: Game<T>
}