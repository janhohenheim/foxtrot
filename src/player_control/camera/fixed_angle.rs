use crate::player_control::actions::CameraActions;
use crate::player_control::camera::ThirdPersonCamera;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const MIN_DISTANCE: f32 = 5.;
const MAX_DISTANCE: f32 = 20.0;

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct FixedAngleCamera {
    pub transform: Transform,
    pub target: Vec3,
    pub up: Vec3,
    pub secondary_target: Option<Vec3>,
    pub distance: f32,
}

impl Default for FixedAngleCamera {
    fn default() -> Self {
        Self {
            up: Vec3::Y,
            transform: default(),
            distance: MAX_DISTANCE / 2.,
            target: default(),
            secondary_target: default(),
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
        }
    }
}

impl FixedAngleCamera {
    pub fn forward(&self) -> Vec3 {
        // The camera is rotated to always look down,
        // so the forward vector for the player is actually the camera's up vector
        self.transform.up()
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
        if let Some(zoom) = camera_actions.zoom {
            self.zoom(zoom);
        }
        self.follow_target();

        self.get_camera_transform(dt, transform)
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
        let zoom_speed = 0.5;
        let zoom = zoom * zoom_speed;
        self.distance = (self.distance - zoom).clamp(MIN_DISTANCE, MAX_DISTANCE);
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
}
