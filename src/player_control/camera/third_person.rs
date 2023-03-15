use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::{FirstPersonCamera, FixedAngleCamera};
use crate::util::trait_extension::{Vec2Ext, Vec3Ext};
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ThirdPersonCamera {
    pub transform: Transform,
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
            transform: default(),
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
        let eye = target - first_person_camera.forward() * distance;
        let up = first_person_camera.up;
        let eye = Transform::from_translation(eye).looking_at(target, up);
        Self {
            transform: eye,
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
        let mut transform = fixed_angle_camera.transform;
        let config = fixed_angle_camera.config.clone();
        transform.rotate_axis(
            transform.right(),
            config.camera.third_person.most_acute_from_above,
        );
        Self {
            transform,
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
        self.transform.forward()
    }

    pub fn update_transform(
        &mut self,
        camera_actions: &ActionState<CameraAction>,
        rapier_context: &RapierContext,
        rig: &mut Rig,
    ) -> Result<()> {
        if let Some(secondary_target) = self.secondary_target {
            self.align_with_primary_and_secondary_target(secondary_target, rig);
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
        rig.driver_mut::<LookAt>().target = self.target;
        rig.driver_mut::<Position>().position = self.target;
        self.transform.translation = self.target + rig.driver::<Arm>().offset;
        self.transform.rotation = rig.driver_mut::<Rotation>().rotation;

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

    fn align_with_primary_and_secondary_target(&mut self, secondary_target: Vec3, rig: &mut Rig) {
        let target_to_secondary_target = (secondary_target - self.target).split(self.up).horizontal;
        if target_to_secondary_target.is_approx_zero() {
            return;
        }
        let target_to_secondary_target = target_to_secondary_target;
        let eye_to_target = (self.target - self.transform.translation)
            .split(self.up)
            .horizontal;
        let yaw = eye_to_target
            .angle_between(target_to_secondary_target)
            .to_degrees();
        let yaw_pitch = rig.driver_mut::<YawPitch>();
        yaw_pitch.rotate_yaw_pitch(yaw, 0.);
    }

    fn set_arm(&mut self, rapier_context: &RapierContext, rig: &mut Rig) {
        let line_of_sight_result = self.keep_line_of_sight(rapier_context);
        let translation_smoothing =
            if line_of_sight_result.correction == LineOfSightCorrection::Further {
                self.config
                    .camera
                    .third_person
                    .translation_smoothing_going_further
            } else {
                self.config
                    .camera
                    .third_person
                    .translation_smoothing_going_closer
            };
        rig.driver_mut::<Arm>().offset = line_of_sight_result.offset;
        //rig.driver_mut::<Smooth>().position_smoothness = translation_smoothing;
    }

    pub fn keep_line_of_sight(&self, rapier_context: &RapierContext) -> LineOfSightResult {
        let origin = self.target;
        let direction = -self.forward();

        let distance = self.get_raycast_distance(origin, direction, rapier_context);
        let offset = direction * distance;

        let original_distance = self.target - self.transform.translation;
        let correction = if distance * distance < original_distance.length_squared() - 1e-3 {
            LineOfSightCorrection::Closer
        } else {
            LineOfSightCorrection::Further
        };
        LineOfSightResult { offset, correction }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineOfSightResult {
    pub offset: Vec3,
    pub correction: LineOfSightCorrection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineOfSightCorrection {
    Closer,
    Further,
}
