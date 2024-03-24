use crate::entity::camera::Camera;
use crate::entity::entityv2::Entity3d;
use crate::entity::structures::{Transform, Vector3d};
use crate::game::scene::{LevelTrait, Scene};
use crate::graphics::object::EntityData;
use crate::graphics::opengl_util::create_vertex_and_fragment_shaders;
use crate::graphics::openglv2::{Program, VertexShader, ShaderParam, ShaderType, Vao, FragmentShader};
use crate::graphics::openglv2::ShaderParamType::Vec4;
use crate::io::loader::{read_fragment_shader_source, read_fragment_shaders_into_memory, read_vertex_shader_source, read_vertex_shaders_into_memory};

pub struct Level;

pub fn shader_params() -> ShaderParam {
    ShaderParam{
        attribute_name: String::from("a_position"),
        param_type: Vec4
    }
}

struct BasicCube {
    pub entity3d: Entity3d
}

impl BasicCube {
    pub fn create() -> Self {
        let vertex_shader = VertexShader::create(
            read_vertex_shader_source("basic.vert"),
            Some(vec![shader_params()]));

        let fragment_shader = FragmentShader::create(
            read_fragment_shader_source("basic.frag"));

        let entity_data = EntityData::from_wavefront_object_file("cube.obj");
        let transform = Transform::create_zero_transform();

        let program = Program::create(&vertex_shader, &fragment_shader);

        let vao = Vao::create(&program, &entity_data.vertices, &vertex_shader.shader_params);

        Self {
            entity3d: Entity3d::create(program, vao, transform, entity_data)
        }

    }
}

impl LevelTrait for Level {
    fn load() -> Scene {
        let (vertex_shader, fragment_shader)
            = create_vertex_and_fragment_shaders("textured2", vec!(shader_params()));

        let camera = Camera::create(
            1.0,
            1000.0,
            10.0,
            Vector3d { x: 5.0, y: 0.0, z: 0.0 },
            Vector3d{ x: 0.0, y: 0.0, z: 180.0 });

        let cube = BasicCube::create();

        Scene {
            cameras: vec![camera],
            entities: vec![cube.entity3d]
        }
    }
}
