use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Rotation3, Unit, Vector3};
use crate::entity::camera::Camera;
use crate::entity::entity::Entity3d;
use crate::entity::structures::Vector3d;

fn to_point3(vector_3d: &Vector3d) -> Point3<f32> {
    Point3::new(vector_3d.x, vector_3d.y, vector_3d.z)
}

pub fn apply_3d_transformations(entity_3d: &Entity3d, camera: &Camera) -> Matrix4<f32> {
    let eye = to_point3(&camera.transform.position);
    let target = to_point3(&camera.target);
    let view = Matrix4::look_at_rh(&eye, &target, &Vector3::y());

    let translation_vector = Vector3::new(entity_3d.transform.position.x, entity_3d.transform.position.y, entity_3d.transform.position.z);
    let translation_matrix = Matrix4::new_translation(&translation_vector);

    let rotation_vector = Vector3::new(entity_3d.transform.rotation.x, entity_3d.transform.rotation.y, entity_3d.transform.rotation.z);
    let rotation_matrix = Matrix4::from_scaled_axis(rotation_vector);

    let scaling_vector = &Vector3::new(entity_3d.transform.scale.x, entity_3d.transform.scale.y, entity_3d.transform.scale.z);
    let scale_matrix = Matrix4::new_nonuniform_scaling(scaling_vector);

    let projection = Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0);
    let model_view = view * translation_matrix;

    let model_view_projection = projection.as_matrix() * model_view * rotation_matrix * scale_matrix;
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