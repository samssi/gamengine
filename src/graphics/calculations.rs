use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Rotation3, Unit, Vector3};
use crate::entity::camera::Camera;
use crate::entity::entity::Entity3d;
use crate::entity::structures::Vector3d;
use crate::state::context::GameContext;

fn to_point3(vector_3d: &Vector3d) -> Point3<f32> {
    Point3::new(vector_3d.x, vector_3d.y, vector_3d.z)
}

fn to_vector3(vector_3d: &Vector3d) -> Vector3<f32> {
    Vector3::new(vector_3d.x, vector_3d.y, vector_3d.z)
}

pub fn apply_3d_transformations_perspective<T>(game_context: &GameContext<T>, entity_3d: &Entity3d, camera: &Camera) -> Matrix4<f32> {
    /*
        let eye = Point3::new(0.0, 0.0, 600.0);
        let target = Point3::new(0.0, 0.0, 0.0);
     */
    //let eye = to_point3(&camera.transform.position);
    /*let eye = Point3::new(0.0, 0.0, 600.0);
    println!("{:?}", &camera.transform.rotation);
    let rotation_matrix = Matrix4::from_scaled_axis(Vector3::new(camera.transform.rotation.y, camera.transform.rotation.x, camera.transform.rotation.z));

    //let translation_vec = to_vector3(&camera.transform.rotation);
    //let translation_mat = Matrix4::new_translation(&translation_vec);

    let target = to_point3(&camera.transform.position);
    //let target =

    let view = Matrix4::look_at_rh(&eye, &target, &Vector3::y());*/
    /*
    let eye_rotation_vector = to_vector3(&camera.transform.rotation);
    let eye_rotation_matrix = Matrix4::from_scaled_axis(eye_rotation_vector);
     */

    /*
    let eye_translation_vector = to_vector3(&camera.target) + to_vector3(&camera.transform.position);
    let eye_translation_matrix = Matrix4::new_translation(&eye_translation_vector);

    let camera_rotation_vector = to_vector3(&camera.transform.rotation);
    // TODO: this is radians
    let camera_rotation_matrix = Matrix4::from_scaled_axis(camera_rotation_vector);

    let eye_view_matrix = (eye_translation_matrix * camera_rotation_matrix);

    let foo = Matrix4::from_data(eye_view_matrix.data);

    let up = Vector3::y();
    let eye = Vector3::try_from(foo).unwrap();

    let target = to_point3(&camera.target) + to_vector3(&camera.transform.position);

    let view = Matrix4::face_towards(&eye, &target, &up);*/

    //let view = Matrix4::look_at_rh(&eye, &target, &up);

    let eye = to_point3(&camera.transform.position);
    // let target = to_point3(&camera.target);
    let target = to_point3(&camera.target) + to_vector3(&camera.transform.position);


    let view = Matrix4::look_at_rh(&eye, &target, &Vector3::y());

    let translation_vector = Vector3::new(entity_3d.transform.position.x, entity_3d.transform.position.y, entity_3d.transform.position.z);
    let translation_matrix = Matrix4::new_translation(&translation_vector);

    let rotation_vector = Vector3::new(entity_3d.transform.rotation.x, entity_3d.transform.rotation.y, entity_3d.transform.rotation.z);
    // TODO: this is radians
    let rotation_matrix = Matrix4::from_scaled_axis(rotation_vector);

    let scaling_vector = &Vector3::new(entity_3d.transform.scale.x, entity_3d.transform.scale.y, entity_3d.transform.scale.z);
    let scale_matrix = Matrix4::new_nonuniform_scaling(scaling_vector);

    let aspect = game_context.window_context.window_properties.width as f32 / game_context.window_context.window_properties.height as f32;
    let projection = Perspective3::new(aspect, 3.14 / 2.0, 1.0, 1000.0);
    let model_view = view * translation_matrix * rotation_matrix * scale_matrix;

    let model_view_projection = projection.as_matrix() * model_view;
    model_view_projection
}

pub fn apply_3d_transformations_ortho(entity_3d: &Entity3d) -> Matrix4<f32> {
    let left = -1.0;
    let right = 1.0;
    let bottom = -1.0;
    let top = 1.0;
    let near = 0.1;
    let far = 100.0;

    let projection_matrix = Matrix4::new_orthographic(left, right, bottom, top, near, far);

    // Look-At Transformation
    let eye = Point3::new(0.0, 0.0, 5.0);
    let target = Point3::origin();
    let up = Vector3::y();

    let view_matrix = Matrix4::look_at_rh(&eye, &target, &up);

    // Translation
    let translation_vector = Vector3::new(entity_3d.transform.position.x, entity_3d.transform.position.y, entity_3d.transform.position.z);
    let translation_matrix = Matrix4::new_translation(&translation_vector);

    // Rotation
    let axis_of_rotation = Unit::new_normalize(Vector3::x());
    let angle_of_rotation = 45.0f32.to_radians();
    let rotation_matrix = Rotation3::from_axis_angle(&axis_of_rotation, angle_of_rotation);

    let model_matrix = translation_matrix * rotation_matrix.to_homogeneous();

    let final_matrix = projection_matrix * view_matrix * model_matrix;
    final_matrix
}