use crate::entity::camera::Camera;
use crate::entity::entityv2::Entity3d;
use crate::entity::structures::{Transform, Vector3d};
use crate::game::scene::{LevelTrait, Scene};
use crate::graphics::object::EntityData;
use crate::graphics::openglv2::{Program, Shader, ShaderParam, ShaderType, Vao};
use crate::graphics::openglv2::ShaderParamType::Vec4;
use crate::io::loader::{read_fragment_shader_source, read_fragment_shaders_into_memory, read_vertex_shader_source, read_vertex_shaders_into_memory};

pub struct Level;

impl LevelTrait for Level {
    fn load() -> Scene {
        let vertex_shader = Shader::create(
            ShaderType::VertexShader, read_vertex_shader_source("basic.vert"),
            Some(vec![
                ShaderParam{
                    attribute_name: String::from("a_position"),
                    param_type: Vec4
                }
            ]));

        let fragment_shader = Shader::create(
            ShaderType::FragmentShader, read_fragment_shader_source("basic.frag"), None);

        let program = Program::create(&vertex_shader, &fragment_shader);
        let entity_data = EntityData::from_wavefront_object_file("cube.obj");
        let transform = Transform::create_zero_transform();
        let camera = Camera::create(
            1.0,
            1000.0,
            10.0,
            Vector3d { x: 5.0, y: 0.0, z: 0.0 },
            Vector3d{ x: 0.0, y: 0.0, z: 180.0 });

        let vao = Vao::create(&program, &entity_data.vertices, &vertex_shader.shader_params);

        let cube = Entity3d::create(program, vao, transform, entity_data);

        Scene {
            cameras: vec![camera],
            entities: vec![cube]
        }
    }
}
