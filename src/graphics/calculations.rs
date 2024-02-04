use nalgebra::{Matrix4, Point3, Rotation3, Unit, Vector3};
use crate::entity::entity::Entity3d;

pub fn apply_3d_transformations(entity_3d: &Entity3d) -> Matrix4<f32> {
    // TODO: uses atm only orthographic projection
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

    let model_matrix = translation_matrix; //* rotation_matrix.to_homogeneous();

    let final_matrix = projection_matrix * view_matrix * model_matrix;
    final_matrix
}