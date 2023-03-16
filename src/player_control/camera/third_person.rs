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
    for (mut camera, mut rig, actions, transform) in camera_query
        .iter_mut()
        .filter(|query_result| query_result.0.kind == IngameCameraKind::ThirdPerson)
    {
        rig.driver_mut::<LookAt>().target = camera.target.translation;
        rig.driver_mut::<Position>().position = camera.target.translation;

        let camera_movement = actions
            .axis_pair(CameraAction::Pan)
            .context("Camera movement is not an axis pair")?
            .xy();
        if !camera_movement.is_approx_zero() {
            let yaw = -camera_movement.x * config.camera.mouse_sensitivity_x;
            let pitch = -camera_movement.y * config.camera.mouse_sensitivity_y;
            let yaw_pitch = rig.driver_mut::<YawPitch>();
            yaw_pitch.rotate_yaw_pitch(yaw.to_degrees(), pitch.to_degrees());
            yaw_pitch.pitch_degrees = clamp_pitch_degrees(
                camera.target.up(),
                transform.forward(),
                yaw_pitch.pitch_degrees,
                config.camera.third_person.min_degrees_looking_down,
                config.camera.third_person.min_degrees_looking_up,
            );
        }

        set_desired_distance(&mut camera, actions, &config);

        let distance = get_distance_to_collision(&rapier_context, &config, &camera, transform);
        let zoom_smoothness = get_zoom_smoothness(&config, &mut rig, distance);
        set_arm(&mut rig, distance, zoom_smoothness, dt);

        rig.driver_mut::<Smooth>().position_smoothness =
            config.camera.third_person.translation_smoothing;
        rig.driver_mut::<Smooth>().rotation_smoothness =
            config.camera.third_person.rotation_smoothing;
        rig.driver_mut::<LookAt>().smoothness = config.camera.third_person.tracking_smoothing;
    }
    Ok(())
}

fn get_zoom_smoothness(config: &GameConfig, rig: &Rig, new_distance: f32) -> f32 {
    let current_distance = rig.driver::<Arm>().offset.z;
    if new_distance < current_distance - 1e-4 {
        config.camera.third_person.zoom_in_smoothing
    } else {
        config.camera.third_person.zoom_out_smoothing
    }
}

fn set_arm(rig: &mut Rig, distance: f32, zoom_smoothness: f32, dt: f32) {
    // Taken from https://github.com/h3r2tic/dolly/blob/main/src/util.rs#L34
    const SMOOTHNESS_MULT: f32 = 8.0;
    let factor = 1.0 - (-SMOOTHNESS_MULT * dt / zoom_smoothness.max(1e-5)).exp();
    let arm_length = &mut rig.driver_mut::<Arm>().offset.z;
    *arm_length = arm_length.lerp(distance, factor);
}

fn set_desired_distance(
    camera: &mut IngameCamera,
    actions: &ActionState<CameraAction>,
    config: &GameConfig,
) {
    let zoom = actions.clamped_value(CameraAction::Zoom) * config.camera.third_person.zoom_speed;

    camera.desired_distance = (camera.desired_distance - zoom).clamp(
        config.camera.third_person.min_distance,
        config.camera.third_person.max_distance,
    );
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

    let min_distance_to_objects = config.camera.third_person.min_distance_to_objects;
    rapier_context
        .cast_ray(origin, direction, max_toi, solid, filter)
        .map(|(_entity, toi)| toi - min_distance_to_objects)
        .unwrap_or(max_toi)
}
