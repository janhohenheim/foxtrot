use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::ThirdPersonCamera;
use anyhow::Result;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct FixedAngleCamera {
    pub transform: Transform,
    pub target: Vec3,
    pub up: Vec3,
    pub secondary_target: Option<Vec3>,
    pub distance: f32,
    pub config: GameConfig,
}

impl Default for FixedAngleCamera {
    fn default() -> Self {
        Self {
            up: Vec3::Y,
            transform: default(),
            distance: 1.,
            target: default(),
            secondary_target: default(),
            config: default(),
        }
    }
}

impl From<&ThirdPersonCamera> for FixedAngleCamera {
    fn from(third_person_camera: &ThirdPersonCamera) -> Self {
        Self {
            transform: third_person_camera.transform,
            target: third_person_camera.target,
            up: third_person_camera.up,
            distance: third_person_camera.distance,
            secondary_target: third_person_camera.secondary_target,
            config: third_person_camera.config.clone(),
        }
    }
}

impl FixedAngleCamera {
    pub fn forward(&self) -> Vec3 {
        // The camera is rotated to always look down,
        // so the forward vector for the player is actually the camera's up vector
        self.transform.up()
    }

    pub fn update_transform(
        &mut self,
        dt: f32,
        camera_actions: &ActionState<CameraAction>,
        transform: Transform,
    ) -> Result<Transform> {
        let zoom = camera_actions.clamped_value(CameraAction::Zoom);
        self.zoom(zoom);
        self.follow_target();
        Ok(self.get_camera_transform(dt, transform))
    }

    fn follow_target(&mut self) {
        let target = if let Some(secondary_target) = self.secondary_target {
            (self.target + secondary_target) / 2.
        } else {
            self.target
        };
        self.transform.translation = target + self.up * self.distance;
        self.transform.look_at(target, self.transform.up());
    }

    fn zoom(&mut self, zoom: f32) {
        let zoom_speed = self.config.camera.fixed_angle.zoom_speed;
        let zoom = zoom * zoom_speed;
        let min_distance = self.config.camera.fixed_angle.min_distance;
        let max_distance = self.config.camera.fixed_angle.max_distance;
        self.distance = (self.distance - zoom).clamp(min_distance, max_distance);
    }

    fn get_camera_transform(&self, dt: f32, mut transform: Transform) -> Transform {
        let translation_smoothing = self.config.camera.fixed_angle.translation_smoothing;
        let scale = (translation_smoothing * dt).min(1.);
        transform.translation = transform
            .translation
            .lerp(self.transform.translation, scale);

        let rotation_smoothing = self.config.camera.fixed_angle.rotation_smoothing;
        let scale = (rotation_smoothing * dt).min(1.);
        transform.rotation = transform.rotation.slerp(self.transform.rotation, scale);

        transform
    }
}
