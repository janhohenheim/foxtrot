use crate::player_control::actions::Actions;
use crate::util::trait_extension::{Vec2Ext, Vec3Ext};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::{PI, TAU};

const MAX_DISTANCE: f32 = 5.0;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ThirdPersonCamera {
    pub eye: Transform,
    pub target: Vec3,
    pub up: Vec3,
    last_eye: Transform,
    last_target: Vec3,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            eye: default(),
            target: default(),
            last_eye: default(),
            last_target: default(),
            up: Vec3::Y,
        }
    }
}

impl ThirdPersonCamera {
    pub fn forward(&self) -> Vec3 {
        self.eye.forward()
    }
}

pub fn init_camera_eye(
    mut camera_query: Query<(&Transform, &mut ThirdPersonCamera), Added<ThirdPersonCamera>>,
) {
    for (transform, mut camera) in &mut camera_query {
        camera.last_eye = *transform;
    }
}

pub fn follow_target(mut camera_query: Query<&mut ThirdPersonCamera>) {
    for mut camera in &mut camera_query {
        let target_movement = (camera.target - camera.last_target).collapse_approx_zero();
        camera.eye.translation = camera.last_eye.translation + target_movement;

        let new_target = camera.target;
        if !(new_target - camera.eye.translation).is_approx_zero() {
            let up = camera.up;
            camera.eye.look_at(new_target, up);
        }
    }
}

pub fn handle_camera_controls(
    mut camera_query: Query<&mut ThirdPersonCamera>,
    actions: Res<Actions>,
) {
    let mouse_sensitivity = 1e-2;
    let camera_movement = match actions.camera_movement {
        Some(vector) => vector * mouse_sensitivity,
        None => return,
    };

    if camera_movement.is_approx_zero() {
        return;
    }

    for mut camera in camera_query.iter_mut() {
        let yaw = -camera_movement.x.clamp(-PI, PI);
        let yaw_rotation = Quat::from_axis_angle(camera.up, yaw);

        let pitch = -camera_movement.y;
        let pitch = clamp_pitch(&camera, pitch);
        let pitch_rotation = Quat::from_axis_angle(camera.eye.local_x(), pitch);

        let pivot = camera.target;
        let rotation = yaw_rotation * pitch_rotation;
        camera.eye.rotate_around(pivot, rotation);
    }
}

pub fn clamp_pitch(camera: &ThirdPersonCamera, angle: f32) -> f32 {
    const MOST_ACUTE_ALLOWED_FROM_ABOVE: f32 = TAU / 10.;
    const MOST_ACUTE_ALLOWED_FROM_BELOW: f32 = TAU / 7.;

    let forward = camera.eye.forward();
    let up = camera.up;
    let angle_to_axis = forward.angle_between(up);
    let (acute_angle_to_axis, most_acute_allowed, sign) = if angle_to_axis > PI / 2. {
        (PI - angle_to_axis, MOST_ACUTE_ALLOWED_FROM_ABOVE, -1.)
    } else {
        (angle_to_axis, MOST_ACUTE_ALLOWED_FROM_BELOW, 1.)
    };
    let new_angle = if acute_angle_to_axis < most_acute_allowed {
        angle - sign * (most_acute_allowed - acute_angle_to_axis)
    } else {
        angle
    };
    if new_angle.abs() < 0.01 {
        0.
    } else {
        new_angle
    }
}

pub fn update_camera_transform(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut ThirdPersonCamera)>,
    rapier_context: Res<RapierContext>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut camera) in camera_query.iter_mut() {
        let line_of_sight_result = keep_line_of_sight(&camera, &rapier_context);
        let translation_smoothing =
            if line_of_sight_result.correction == LineOfSightCorrection::Closer {
                25.
            } else {
                10.
            };
        let direction = line_of_sight_result.location - transform.translation;
        let scale = (translation_smoothing * dt).max(1.);
        transform.translation += direction * scale;

        let rotation_smoothing = 15.;
        let scale = (rotation_smoothing * dt).max(1.);
        transform.rotation = transform.rotation.slerp(camera.eye.rotation, scale);

        camera.last_eye = camera.eye;
        camera.last_target = camera.target;
    }
}

pub fn keep_line_of_sight(
    camera: &ThirdPersonCamera,
    rapier_context: &RapierContext,
) -> LineOfSightResult {
    let origin = camera.target;
    let desired_direction = camera.eye.translation - camera.target;
    let norm_direction = desired_direction.try_normalize().unwrap_or(Vec3::Z);

    let distance = get_raycast_distance(origin, norm_direction, rapier_context, MAX_DISTANCE);
    let location = origin + norm_direction * distance;
    let correction = if distance * distance < desired_direction.length_squared() {
        LineOfSightCorrection::Closer
    } else {
        LineOfSightCorrection::Further
    };
    LineOfSightResult {
        location,
        correction,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineOfSightResult {
    pub location: Vec3,
    pub correction: LineOfSightCorrection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineOfSightCorrection {
    Closer,
    Further,
}

pub fn get_raycast_distance(
    origin: Vec3,
    direction: Vec3,
    rapier_context: &RapierContext,
    max_distance: f32,
) -> f32 {
    let max_toi = max_distance;
    let solid = true;
    let mut filter = QueryFilter::only_fixed();
    filter.flags |= QueryFilterFlags::EXCLUDE_SENSORS;

    let min_distance_to_objects = 0.01;
    rapier_context
        .cast_ray(origin, direction, max_toi, solid, filter)
        .map(|(_entity, toi)| toi - min_distance_to_objects)
        .unwrap_or(max_distance)
}
