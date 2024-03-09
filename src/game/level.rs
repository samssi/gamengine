use crate::entity::camera::Camera;
use crate::entity::entity::Entity3d;
use crate::entity::structures::{Transform, Vector3d};
use crate::io::object::{get_geometric_vertices, get_texture_points};
use crate::state::context::{GameContext, ObjectContext, ShaderContext};

pub struct Scene {
    pub entities: Vec<Entity3d>,
    pub cameras: Vec<Camera>
}

pub trait Level {
    fn load(object_context: &ObjectContext, shader_context: &ShaderContext) -> Scene;
}

fn create_cameras() -> Vec<Camera> {
    vec![Camera{
        transform: Transform::new_transform_with_position_and_rotation(
            Vector3d { x: 1.7, y: 0.0, z: 0.0 },
            Vector3d{ x: 0.0, y: 0.0, z: 180.0 }
            /*Vector3d { x: 2.0, y: 2.0, z: 0.0 },
            Vector3d{ x: 0.0, y: -1.0, z: 180.0 }*/
        ),
        distance: 100.0,
        near: 1.0,
        far: 1000.0
    }]
}

fn create_leppis_plane(object_context: &ObjectContext, shader_context: &ShaderContext) -> Entity3d {
    let file_content = object_context.objects.get("leppis-plane.obj").expect("object not found");
    let points = get_geometric_vertices(file_content);

    let texture = object_context.textures.get("leppis.png").expect("texture not found");
    let texture_coordinates = get_texture_points(file_content);

    Entity3d::with_transform(
        shader_context,
        texture,
        "textured",
        points,
        Transform::new_transform_with_position_rotation_and_scale(
            Vector3d{x: -1.35, y: 0.85, z: 0.0 },
            Vector3d{x: 90.0, y: 0.0, z: 0.0 },
            Vector3d{x: 1.2, y: 1.2, z: 1.2}
        ),
        texture_coordinates,
    )
}

fn create_desk(object_context: &ObjectContext, shader_context: &ShaderContext) -> Entity3d {
    let file_content = object_context.objects.get("desk.obj").expect("object not found");
    let points = get_geometric_vertices(file_content);

    let texture = object_context.textures.get("lemon.png").expect("texture not found");
    let texture_coordinates = get_texture_points(file_content);

    Entity3d::with_transform(
        shader_context,
        texture,
        "textured",
        points,
        Transform::new_transform_with_position_and_rotation(
            Vector3d{x: -0.9, y: -0.8, z: 0.0 },
            Vector3d{x: 0.0, y: 0.0, z: 0.0 }
        ),
        texture_coordinates,
    )
}

impl Level for Scene {
    fn load(object_context: &ObjectContext, shader_context: &ShaderContext) -> Scene {
        Scene{
            entities: vec![
                create_desk(object_context, shader_context),
                create_leppis_plane(object_context, shader_context)],
            cameras: create_cameras()
        }
    }
}