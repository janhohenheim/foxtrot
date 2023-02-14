use crate::player_control::actions::CameraActions;
use crate::player_control::camera::util::clamp_pitch;
use crate::player_control::camera::ThirdPersonCamera;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct FirstPersonCamera {
    pub transform: Transform,
    pub look_target: Option<Vec3>,
    pub up: Vec3,
}

impl Default for FirstPersonCamera {
    fn default() -> Self {
        Self {
            transform: default(),
            look_target: default(),
            up: Vec3::Y,
        }
    }
}

impl From<&ThirdPersonCamera> for FirstPersonCamera {
    fn from(camera: &ThirdPersonCamera) -> Self {
        Self {
            transform: camera.transform,
            look_target: camera.secondary_target,
            up: camera.up,
        }
    }
}

impl FirstPersonCamera {
    pub fn forward(&self) -> Vec3 {
        self.transform.forward()
    }

    pub fn init_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn update_transform(
        &mut self,
        dt: f32,
        camera_actions: CameraActions,
        transform: Transform,
    ) -> Transform {
        if let Some(look_target) = self.look_target {
            self.look_at(look_target);
        } else if let Some(camera_movement) = camera_actions.movement {
            self.handle_camera_controls(camera_movement);
        }
        self.get_camera_transform(dt, transform)
    }

    fn get_camera_transform(&self, dt: f32, mut transform: Transform) -> Transform {
        let translation_smoothing = 50.;
        let scale = (translation_smoothing * dt).min(1.);
        transform.translation = transform
            .translation
            .lerp(self.transform.translation, scale);

        let rotation_smoothing = 45.;
        let scale = (rotation_smoothing * dt).min(1.);
        transform.rotation = transform.rotation.slerp(self.transform.rotation, scale);

        transform
    }

    fn handle_camera_controls(&mut self, camera_movement: Vec2) {
        let mouse_sensitivity = 1e-3;
        let camera_movement = camera_movement * mouse_sensitivity;

        let yaw = -camera_movement.x.clamp(-PI, PI);
        let pitch = self.clamp_pitch(-camera_movement.y);
        self.rotate(yaw, pitch);
    }

    fn look_at(&mut self, target: Vec3) {
        let up = self.up;
        self.transform.look_at(target, up);
    }

    fn rotate(&mut self, yaw: f32, pitch: f32) {
        let yaw_rotation = Quat::from_axis_angle(self.up, yaw);
        let pitch_rotation = Quat::from_axis_angle(self.transform.local_x(), pitch);

        let rotation = yaw_rotation * pitch_rotation;
        self.transform.rotate(rotation);
    }

    fn clamp_pitch(&self, angle: f32) -> f32 {
        clamp_pitch(self.up, self.forward(), angle)
    }
}
