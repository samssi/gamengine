use crate::entity::camera::Camera;
use crate::entity::entityv2::Entity3d;
use crate::entity::structures::{Transform, Vector3d};
use crate::game::scene::{LevelTrait, Scene};
use crate::graphics::object::EntityData;
use crate::graphics::openglv2::{Program, Shader, ShaderType, Vao};
use crate::io::loader::{read_fragment_shader_source, read_fragment_shaders_into_memory, read_vertex_shader_source, read_vertex_shaders_into_memory};

struct Level;

impl LevelTrait for Level {
    fn load() -> Scene {
        let vertex_shader = Shader::create(ShaderType::VertexShader, read_vertex_shader_source("basic.vert"));
        let fragment_shader = Shader::create(ShaderType::FragmentShader, read_fragment_shader_source("basic.frag"));
        let program = Program::create(&vertex_shader, &fragment_shader);
        let entity_data = EntityData::from_wavefront_object_file("cube.obj");
        let transform = Transform::create_zero_transform();
        let camera = Camera::create(
            1.0,
            1000.0,
            100.0,
            Vector3d { x: 1.7, y: 0.0, z: 0.0 },
            Vector3d{ x: 0.0, y: 0.0, z: 180.0 });

        let cube = Entity3d::create(program, transform, entity_data);

        Scene {
            cameras: vec![camera],
            entities: vec![cube]
        }
    }
}
