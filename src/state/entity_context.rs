use std::collections::HashMap;
use gl::types::GLuint;
use crate::entity::entity::Entity3d;

pub struct ShaderContext {
    pub vertex_shaders: HashMap<String, GLuint>,
    pub fragment_shaders: HashMap<String, GLuint>
}

pub struct EntityContext {
    pub entities: Vec<Entity3d>,
}