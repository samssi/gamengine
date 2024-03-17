use crate::entity::entity::Entity3d;
use crate::entity::structures::{Transform, Vector3d};
use crate::io::wavefront_reader::{get_geometric_vertices, get_texture_points};
use crate::state::context::{ObjectContext, ShaderContext};

fn create_cube(shader_context: &ShaderContext, object_context: &ObjectContext, position: Option<Vector3d>) -> Entity3d {
    let file_content = object_context.objects.get("cube.obj").expect("object not found");
    let points = get_geometric_vertices(file_content);

    let texture = object_context.textures.get("leppis.png").expect("texture not found");
    let texture_coordinates = get_texture_points(file_content);


    match position {
        None => { Entity3d::with_default_transform(
            &shader_context,
            texture,
            "textured",
            points,
            texture_coordinates
        )}
        Some(position) => {
            Entity3d::with_position_and_scale(
                &shader_context,
                texture,
                points,
                texture_coordinates,
                "textured",
                position,
                Vector3d{x: 20.0, y: 20.0, z: 20.0}
            )}
    }
}

fn vector3d_from_to(from: Vector3d, to: Vector3d, scale_factor: f32) -> Vec<Vector3d> {
    let mut result = Vec::new();

    for x in (from.x as i32)..=(to.x as i32) {
        for y in (from.y as i32)..=(to.y as i32) {
            for z in (from.z as i32)..=(to.z as i32) {
                result.push(Vector3d {
                    x: x as f32 * scale_factor,
                    y: y as f32 * scale_factor,
                    z: z as f32 * scale_factor,
                });
            }
        }
    }

    result
}

pub fn generate_cube_space(shader_context: &ShaderContext, object_context: &ObjectContext) -> Vec<Entity3d> {
    let cube_positions = vector3d_from_to(Vector3d{x: -1.0, y: -1.0, z: -1.0}, Vector3d{x: 1.0, y: 1.0, z: 1.0}, 100.0);
    let cubes: Vec<Entity3d> = cube_positions.iter().map(|cube_position| {
        create_cube(&shader_context, &object_context, Some(cube_position.clone()))
    }).collect();
    cubes
}

pub fn single_cube(shader_context: &ShaderContext, object_context: &ObjectContext) -> Vec<Entity3d> {
    println!("{:?}", vector3d_from_to(Vector3d{x: -1.0, y: -1.0, z: -1.0}, Vector3d{x: 1.0, y: 1.0, z: 1.0}, 100.0));

    vec![create_cube(shader_context, object_context, None)]
}