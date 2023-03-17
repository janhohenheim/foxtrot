use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::util::clamp_pitch_degrees;
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::util::trait_extension::{F32Ext, Vec2Ext};
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn update_rig(
    time: Res<Time>,
    mut camera_query: Query<(
        &mut IngameCamera,
        &mut Rig,
        &ActionState<CameraAction>,
        &Transform,
    )>,
    rapier_context: Res<RapierContext>,
    config: Res<GameConfig>,
) -> Result<()> {
    let dt = time.delta_seconds();
    for (mut camera, mut rig, actions, transform) in camera_query.iter_mut() {
        rig.driver_mut::<LookAt>().target = camera.target.translation;
        rig.driver_mut::<Position>().position = camera.target.translation;

        let camera_movement = actions
            .axis_pair(CameraAction::Pan)
            .context("Camera movement is not an axis pair")?
            .xy();
        if !camera_movement.is_approx_zero() {
            let yaw_pitch = rig.driver_mut::<YawPitch>();
            if camera.kind != IngameCameraKind::FixedAngle {
                let yaw = -camera_movement.x * config.camera.mouse_sensitivity_x;
                let pitch = -camera_movement.y * config.camera.mouse_sensitivity_y;
                yaw_pitch.rotate_yaw_pitch(yaw.to_degrees(), pitch.to_degrees());
                let (most_acute_looking_down, most_acute_looking_up) = match camera.kind {
                    IngameCameraKind::ThirdPerson => (
                        config.camera.third_person.min_degrees_looking_down,
                        config.camera.third_person.min_degrees_looking_up,
                    ),
                    IngameCameraKind::FirstPerson => (
                        config.camera.first_person.min_degrees_looking_down,
                        config.camera.first_person.min_degrees_looking_up,
                    ),
                    _ => unreachable!(),
                };
                yaw_pitch.pitch_degrees = clamp_pitch_degrees(
                    camera.target.up(),
                    transform.forward(),
                    yaw_pitch.pitch_degrees,
                    most_acute_looking_down,
                    most_acute_looking_up,
                );
            } else {
                yaw_pitch.yaw_degrees = 0.;
                yaw_pitch.pitch_degrees = -90.;
            }
        }

        if camera.kind != IngameCameraKind::FirstPerson {
            set_desired_distance(&mut camera, actions, &config);
            let distance = get_distance_to_collision(&rapier_context, &config, &camera, transform);
            let zoom_smoothness = get_zoom_smoothness(&config, &camera, &mut rig, distance);
            set_arm(&mut rig, distance, zoom_smoothness, dt);
        } else {
            rig.driver_mut::<Arm>().offset.z = 0.;
        }

        set_smoothness(&mut rig, &config, &camera);
    }
    Ok(())
}

fn set_smoothness(rig: &mut Rig, config: &GameConfig, camera: &IngameCamera) {
    match camera.kind {
        IngameCameraKind::ThirdPerson => {
            rig.driver_mut::<Smooth>().position_smoothness =
                config.camera.third_person.translation_smoothing;
            rig.driver_mut::<Smooth>().rotation_smoothness =
                config.camera.third_person.rotation_smoothing;
            rig.driver_mut::<LookAt>().smoothness = config.camera.third_person.tracking_smoothing;
        }
        IngameCameraKind::FirstPerson => {
            rig.driver_mut::<Smooth>().position_smoothness =
                config.camera.first_person.translation_smoothing;
            rig.driver_mut::<Smooth>().rotation_smoothness =
                config.camera.first_person.rotation_smoothing;
            rig.driver_mut::<LookAt>().smoothness = config.camera.first_person.tracking_smoothing;
        }
        IngameCameraKind::FixedAngle => {
            rig.driver_mut::<Smooth>().position_smoothness =
                config.camera.fixed_angle.translation_smoothing;
            rig.driver_mut::<Smooth>().rotation_smoothness =
                config.camera.fixed_angle.rotation_smoothing;
        }
    }
}

fn get_zoom_smoothness(
    config: &GameConfig,
    camera: &IngameCamera,
    rig: &Rig,
    new_distance: f32,
) -> f32 {
    let current_distance = rig.driver::<Arm>().offset.z;
    if new_distance < current_distance - 1e-4 {
        match camera.kind {
            IngameCameraKind::ThirdPerson => config.camera.third_person.zoom_in_smoothing,
            IngameCameraKind::FixedAngle => config.camera.fixed_angle.zoom_in_smoothing,
            _ => unreachable!(),
        }
    } else {
        match camera.kind {
            IngameCameraKind::ThirdPerson => config.camera.third_person.zoom_out_smoothing,
            IngameCameraKind::FixedAngle => config.camera.fixed_angle.zoom_out_smoothing,
            _ => unreachable!(),
        }
    }
}

fn set_arm(rig: &mut Rig, distance: f32, zoom_smoothness: f32, dt: f32) {
    // Taken from https://github.com/h3r2tic/dolly/blob/main/src/util.rs#L34
    const SMOOTHNESS_MULTIPLIER: f32 = 8.0;
    let factor = 1.0 - (-SMOOTHNESS_MULTIPLIER * dt / zoom_smoothness.max(1e-5)).exp();
    let arm_length = &mut rig.driver_mut::<Arm>().offset.z;
    *arm_length = arm_length.lerp(distance, factor);
}

fn set_desired_distance(
    camera: &mut IngameCamera,
    actions: &ActionState<CameraAction>,
    config: &GameConfig,
) {
    let zoom = actions.clamped_value(CameraAction::Zoom) * config.camera.third_person.zoom_speed;
    let (min_distance, max_distance) = match camera.kind {
        IngameCameraKind::ThirdPerson => (
            config.camera.third_person.min_distance,
            config.camera.third_person.max_distance,
        ),
        IngameCameraKind::FixedAngle => (
            config.camera.fixed_angle.min_distance,
            config.camera.fixed_angle.max_distance,
        ),
        _ => unreachable!(),
    };
    camera.desired_distance = (camera.desired_distance - zoom).clamp(min_distance, max_distance);
}

fn get_distance_to_collision(
    rapier_context: &RapierContext,
    config: &GameConfig,
    camera: &IngameCamera,
    camera_transform: &Transform,
) -> f32 {
    let origin = camera.target.translation;
    let direction = camera_transform.back();

    let max_toi = camera.desired_distance;
    let solid = true;
    let mut filter = QueryFilter::only_fixed();
    filter.flags |= QueryFilterFlags::EXCLUDE_SENSORS;

    let min_distance = match camera.kind {
        IngameCameraKind::ThirdPerson => config.camera.third_person.min_distance_to_objects,
        IngameCameraKind::FixedAngle => config.camera.fixed_angle.min_distance_to_objects,
        _ => unreachable!(),
    };

    rapier_context
        .cast_ray(origin, direction, max_toi, solid, filter)
        .map(|(_entity, toi)| toi - min_distance)
        .unwrap_or(max_toi)
}
