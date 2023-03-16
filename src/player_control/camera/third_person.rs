use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::{FirstPersonCamera, FixedAngleCamera};
use crate::util::trait_extension::{F32Ext, Vec2Ext, Vec3Ext};
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ThirdPersonCamera {
    pub forward: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub secondary_target: Option<Vec3>,
    pub distance: f32,
    pub config: GameConfig,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            up: Vec3::Y,
            forward: default(),
            distance: 5.,
            target: default(),
            secondary_target: default(),
            config: default(),
        }
    }
}

impl From<&FirstPersonCamera> for ThirdPersonCamera {
    fn from(first_person_camera: &FirstPersonCamera) -> Self {
        let target = first_person_camera.transform.translation;
        let distance = first_person_camera.config.camera.third_person.min_distance;
        let up = first_person_camera.up;
        Self {
            forward: first_person_camera.forward(),
            target,
            up,
            distance,
            secondary_target: first_person_camera.look_target,
            config: first_person_camera.config.clone(),
        }
    }
}

impl From<&FixedAngleCamera> for ThirdPersonCamera {
    fn from(fixed_angle_camera: &FixedAngleCamera) -> Self {
        Self {
            forward: fixed_angle_camera.forward(),
            target: fixed_angle_camera.target,
            up: fixed_angle_camera.up,
            distance: fixed_angle_camera.distance,
            secondary_target: fixed_angle_camera.secondary_target,
            config: fixed_angle_camera.config.clone(),
        }
    }
}

impl ThirdPersonCamera {
    pub fn forward(&self) -> Vec3 {
        self.forward
    }

    pub fn update_rig(
        &mut self,
        camera_actions: &ActionState<CameraAction>,
        rapier_context: &RapierContext,
        rig: &mut Rig,
    ) -> Result<()> {
        if let Some(secondary_target) = self.secondary_target {
            rig.driver_mut::<LookAt>().target = secondary_target;
            rig.driver_mut::<Position>().position = secondary_target;
        } else {
            rig.driver_mut::<LookAt>().target = self.target;
            rig.driver_mut::<Position>().position = self.target;
        }

        let camera_movement = camera_actions
            .axis_pair(CameraAction::Pan)
            .context("Camera movement is not an axis pair")?
            .xy();
        if !camera_movement.is_approx_zero() {
            self.set_yaw_pitch(camera_movement, rig);
        }

        let zoom = camera_actions.clamped_value(CameraAction::Zoom);
        self.update_desired_distance(zoom);
        self.set_arm(rapier_context, rig);
        self.forward = -rig.driver::<Arm>().offset.normalize();
        Ok(())
    }

    fn set_yaw_pitch(&mut self, camera_movement: Vec2, rig: &mut Rig) {
        let yaw = -camera_movement.x * self.config.camera.mouse_sensitivity_x;
        let pitch = -camera_movement.y * self.config.camera.mouse_sensitivity_y;
        let yaw_pitch = rig.driver_mut::<YawPitch>();
        yaw_pitch.rotate_yaw_pitch(yaw.to_degrees(), pitch.to_degrees());
    }

    fn update_desired_distance(&mut self, zoom: f32) {
        let zoom_speed = self.config.camera.third_person.zoom_speed;
        let zoom = zoom * zoom_speed;
        let min_distance = self.config.camera.third_person.min_distance;
        let max_distance = self.config.camera.third_person.max_distance;
        self.distance = (self.distance - zoom).clamp(min_distance, max_distance);
    }

    fn set_arm(&mut self, rapier_context: &RapierContext, rig: &mut Rig) {
        let current_offset = rig.driver::<Arm>().offset;
        let origin = self.target;
        let direction = current_offset.normalize();

        let distance = self.get_raycast_distance(origin, direction, rapier_context);
        let offset = direction * distance;

        let original_distance_squared = current_offset.length_squared();
        let translation_smoothing = if distance.squared() < original_distance_squared - 1e-3 {
            self.config
                .camera
                .third_person
                .translation_smoothing_going_closer
        } else {
            self.config
                .camera
                .third_person
                .translation_smoothing_going_further
        };
        rig.driver_mut::<Arm>().offset = offset;
        //rig.driver_mut::<Smooth>().position_smoothness = translation_smoothing;
    }

    pub fn get_raycast_distance(
        &self,
        origin: Vec3,
        direction: Vec3,
        rapier_context: &RapierContext,
    ) -> f32 {
        let max_toi = self.distance;
        let solid = true;
        let mut filter = QueryFilter::only_fixed();
        filter.flags |= QueryFilterFlags::EXCLUDE_SENSORS;

        let min_distance_to_objects = self.config.camera.third_person.min_distance_to_objects;
        rapier_context
            .cast_ray(origin, direction, max_toi, solid, filter)
            .map(|(_entity, toi)| toi - min_distance_to_objects)
            .unwrap_or(max_toi)
    }
}
