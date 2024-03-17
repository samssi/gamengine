use crate::entity::camera::Camera;
use crate::entity::entityv2::Entity3d;

pub trait LevelTrait {
    fn load() -> Scene;
}

pub struct Scene {
    pub entities: Vec<Entity3d>,
    pub cameras: Vec<Camera>
}
/*
impl Scene {
    pub fn start(level: Level)
    )
}*/