use bevy_rapier3d::na::{Matrix3, Vector3};

pub fn get_rotation_matrix_around_y_axis(angle: f32) -> Matrix3<f32> {
    // See https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
    #[rustfmt::skip]
    Matrix3::new(
        angle.cos(), 0., -angle.sin(),
        0., 1., 0.,
        angle.sin(), 0., angle.cos(),
    )
}

pub fn get_rotation_matrix_around_vector(angle: f32, vector: Vector3<f32>) -> Matrix3<f32> {
    // Source: https://math.stackexchange.com/a/142831/419398
    let u = vector.normalize();
    #[rustfmt::skip]
    let w = Matrix3::new(
        0., -u.z, u.y,
        u.z, 0., -u.x,
        -u.y, u.x, 0.
    );
    Matrix3::identity() + (angle.sin()) * w + (2. * (angle / 2.).sin().powf(2.)) * w.pow(2)
}
