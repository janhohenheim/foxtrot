use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::{PI, TAU};

const MAX_DISTANCE: f32 = 5.0;

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ThirdPersonCamera {
    pub eye: Transform,
    pub target: Vec3,
    pub up: Vec3,
    last_eye: Transform,
    last_target: Vec3,
    secondary_target: Option<Vec3>,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            up: Vec3::Y,
            eye: default(),
            target: default(),
            last_eye: default(),
            last_target: default(),
            secondary_target: default(),
        }
    }
}

impl ThirdPersonCamera {
    pub fn forward(&self) -> Vec3 {
        self.eye.forward()
    }

    pub fn set_secondary_target(&mut self, secondary_target: Vec3) {
        self.secondary_target = Some(secondary_target);
    }

    fn rotate_around_target(&mut self, yaw: f32, pitch: f32) {
        let yaw_rotation = Quat::from_axis_angle(self.up, yaw);
        let pitch_rotation = Quat::from_axis_angle(self.eye.local_x(), pitch);

        let pivot = self.target;
        let rotation = yaw_rotation * pitch_rotation;
        self.eye.rotate_around(pivot, rotation);
    }

    pub fn init_transform(&mut self, transform: Transform) {
        self.last_eye = transform;
    }

    pub fn update_transform(
        &mut self,
        dt: f32,
        camera_movement: Option<Vec2>,
        rapier_context: &RapierContext,
    ) -> Transform {
        self.follow_target();

        if let Some(secondary_target) = self.secondary_target {
            self.move_eye_to_align_target_with(secondary_target);
        } else if let Some(camera_movement) = camera_movement {
            self.handle_camera_controls(camera_movement);
        }

        self.update_camera_transform(dt, rapier_context)
    }

    fn follow_target(&mut self) {
        let target_movement = (self.target - self.last_target).collapse_approx_zero();
        self.eye.translation = self.last_eye.translation + target_movement;

        let new_target = self.target;
        if !(new_target - self.eye.translation).is_approx_zero() {
            let up = self.up;
            self.eye.look_at(new_target, up);
        }
    }

    fn handle_camera_controls(&mut self, camera_movement: Vec2) {
        let mouse_sensitivity = 1e-2;
        let camera_movement = camera_movement * mouse_sensitivity;

        let yaw = -camera_movement.x.clamp(-PI, PI);
        let pitch = self.clamp_pitch(-camera_movement.y);
        self.rotate_around_target(yaw, pitch);
    }

    fn move_eye_to_align_target_with(&mut self, secondary_target: Vec3) {
        let target_to_secondary_target = (secondary_target - self.target).split(self.up).horizontal;
        let eye_to_target = (self.target - self.eye.translation)
            .split(self.up)
            .horizontal;
        let yaw = eye_to_target.angle_between(target_to_secondary_target);
        info!("yaw: {}", yaw);

        self.rotate_around_target(yaw, 0.0);
    }

    fn clamp_pitch(&self, angle: f32) -> f32 {
        const MOST_ACUTE_ALLOWED_FROM_ABOVE: f32 = TAU / 10.;
        const MOST_ACUTE_ALLOWED_FROM_BELOW: f32 = TAU / 7.;

        let forward = self.eye.forward();
        let up = self.up;
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

    fn update_camera_transform(&mut self, dt: f32, rapier_context: &RapierContext) -> Transform {
        let line_of_sight_result = self.keep_line_of_sight(rapier_context);
        let translation_smoothing =
            if line_of_sight_result.correction == LineOfSightCorrection::Closer {
                25.
            } else {
                10.
            };
        let mut transform = self.eye;
        let direction = line_of_sight_result.location - transform.translation;
        let scale = (translation_smoothing * dt).max(1.);
        transform.translation += direction * scale;

        let rotation_smoothing = 15.;
        let scale = (rotation_smoothing * dt).max(1.);
        transform.rotation = transform.rotation.slerp(self.eye.rotation, scale);

        self.last_eye = self.eye;
        self.last_target = self.target;
        transform
    }

    pub fn keep_line_of_sight(&self, rapier_context: &RapierContext) -> LineOfSightResult {
        let origin = self.target;
        let desired_direction = self.eye.translation - self.target;
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
