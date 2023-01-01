use crate::actions::Actions;
use crate::player::Player;
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::na::{Matrix3, Vector3};
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
use std::f32::consts::TAU;

const MAX_DISTANCE: f32 = 6.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
            .add_plugin(LookTransformPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(follow_player.label("follow_player"))
                    .with_system(
                        handle_camera_controls
                            .label("handle_camera_controls")
                            .after("follow_player"),
                    )
                    .with_system(
                        keep_target_visible
                            .label("keep_target_visible")
                            .after("handle_camera_controls"),
                    )
                    .with_system(cursor_grab_system),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    let eye = Vec3::new(MAX_DISTANCE, 0., 0.);
    let target = Vec3::default();
    commands.spawn((
        LookTransformBundle {
            transform: LookTransform::new(eye, target),
            smoother: Smoother::new(0.5), // Value between 0.0 and 1.0, higher is smoother.
        },
        Camera3dBundle::default(),
        Name::new("Camera"),
    ));
}

fn follow_player(
    player_query: Query<(&KinematicCharacterControllerOutput, &Transform), With<Player>>,
    mut camera_query: Query<&mut LookTransform>,
) {
    let (output, transform) = match player_query.iter().next() {
        Some(player) => player,
        None => return,
    };
    let mut camera = match camera_query.iter_mut().next() {
        Some(transform) => transform,
        None => return,
    };

    camera.eye += output.effective_translation;
    camera.target = transform.translation;
}

fn handle_camera_controls(mut camera_query: Query<&mut LookTransform>, actions: Res<Actions>) {
    let mouse_sensitivity = 0.01;
    let mut camera = match camera_query.iter_mut().next() {
        Some(transform) => transform,
        None => return,
    };
    let camera_movement = match actions.camera_movement {
        Some(vector) => vector,
        None => return,
    };

    let direction = camera.look_direction().unwrap_or(Vect::Z);
    let horizontal_rotation_axis = direction.xz().perp();
    let horizontal_rotation_axis =
        Vector3::new(horizontal_rotation_axis.x, 0., horizontal_rotation_axis.y);
    let x_angle = mouse_sensitivity * camera_movement.x;
    let y_angle = -mouse_sensitivity * camera_movement.y;
    let y_angle = clamp_vertical_rotation(direction, y_angle);

    let horizontal_rotation_matrix = get_rotation_matrix_around_y_axis(x_angle);
    let vertical_rotation_matrix =
        get_rotation_matrix_around_vector(y_angle, horizontal_rotation_axis);

    let rotated_direction: Vec3 =
        (vertical_rotation_matrix * horizontal_rotation_matrix * Vector3::from(direction)).into();
    camera.eye = camera.target - rotated_direction * MAX_DISTANCE;
}

fn keep_target_visible(
    mut camera_query: Query<&mut LookTransform>,
    rapier_context: Res<RapierContext>,
) {
    return;
    let mut camera = match camera_query.iter_mut().next() {
        Some(transform) => transform,
        None => return,
    };
    let origin = camera.target;
    let direction = camera.eye - camera.target;
    let max_toi = direction.length();
    let solid = true;
    let filter = QueryFilter::only_fixed();
    if let Some((_entity, toi)) = rapier_context.cast_ray(origin, direction, max_toi, solid, filter)
    {
        let line_of_sight = direction * toi;
        let clamped_line_of_sight = if line_of_sight.length() > MAX_DISTANCE {
            line_of_sight.normalize() * MAX_DISTANCE
        } else {
            line_of_sight
        };
        camera.eye = origin + clamped_line_of_sight;
    }
}

fn clamp_vertical_rotation(current_direction: Vec3, angle: f32) -> f32 {
    let current_angle = current_direction.angle_between(Vect::Y);
    let new_angle = current_angle - angle;

    let max_angle = TAU / (2.0 + 1. / 16.);
    let min_angle = TAU / 16.0;

    let clamped_angle = if new_angle > max_angle {
        max_angle - current_angle
    } else if new_angle < min_angle {
        min_angle - current_angle
    } else {
        angle
    };

    if clamped_angle.abs() < 0.01 {
        // This smooths use experience
        return 0.;
    } else {
        clamped_angle
    }
}

fn get_x_axis_rotation_matrix(angle: f32) -> Matrix3<f32> {
    Matrix3::from_row_iterator(
        #[cfg_attr(rustfmt, rustfmt::skip)]
        [
            1., 0., 0.,
            0., angle.cos(), -angle.sin(),
            0., angle.sin(), angle.cos(),
        ].into_iter(),
    )
}

fn get_rotation_matrix_around_y_axis(angle: f32) -> Matrix3<f32> {
    // See https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
    Matrix3::from_row_iterator(
        #[cfg_attr(rustfmt, rustfmt::skip)]
        [
            angle.cos(), 0., -angle.sin(),
            0., 1., 0.,
            angle.sin(), 0., angle.cos(),
        ].into_iter(),
    )
}

fn get_rotation_matrix_around_vector(angle: f32, vector: Vector3<f32>) -> Matrix3<f32> {
    // Source: https://math.stackexchange.com/a/142831/419398
    let u = vector.normalize();
    let w = Matrix3::from_row_iterator(
        #[cfg_attr(rustfmt, rustfmt::skip)]
        [
            0., -u.z, u.y,
            u.z, 0., -u.x,
            -u.y, u.x, 0.
        ].into_iter(),
    );
    Matrix3::identity() + (angle.sin()) * w + (2. * (angle / 2.).sin().powf(2.)) * w.pow(2)
}
fn cursor_grab_system(mut windows: ResMut<Windows>, key: Res<Input<KeyCode>>) {
    let window = windows.get_primary_mut().unwrap();

    if key.just_pressed(KeyCode::Escape) {
        if matches!(window.cursor_grab_mode(), CursorGrabMode::None) {
            // if you want to use the cursor, but not let it leave the window,
            // use `Confined` mode:
            window.set_cursor_grab_mode(CursorGrabMode::Confined);

            // for a game that doesn't use the cursor (like a shooter):
            // use `Locked` mode to keep the cursor in one place
            window.set_cursor_grab_mode(CursorGrabMode::Locked);
            // also hide the cursor
            window.set_cursor_visibility(false);
        } else {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}
