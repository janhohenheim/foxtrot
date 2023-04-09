use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::rig::arm::{get_arm_distance, get_zoom_smoothness, set_arm};
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::util::trait_extension::Vec2Ext;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

mod arm;

#[sysfail(log(level = "error"))]
pub(crate) fn update_rig(
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
        set_look_at(&mut rig, &camera);
        set_position(&mut rig, &camera);
        if camera.kind == IngameCameraKind::FixedAngle {
            let yaw_pitch = rig.driver_mut::<YawPitch>();
            yaw_pitch.yaw_degrees = 0.;
            yaw_pitch.pitch_degrees = config.camera.fixed_angle.pitch;
        } else {
            let camera_movement = get_camera_movement(actions)?;
            if !camera_movement.is_approx_zero() {
                set_yaw_pitch(&mut rig, &camera, camera_movement, &config);
            }
        }

        set_desired_distance(&mut camera, actions, &config);
        let distance = get_arm_distance(&camera, transform, &rapier_context, &config);
        if let Some(distance) = distance {
            let zoom_smoothness = get_zoom_smoothness(&config, &camera, &rig, distance);
            set_arm(&mut rig, distance, zoom_smoothness, dt);
        }

        set_smoothness(&mut rig, &config, &camera);
    }
    Ok(())
}

fn get_camera_movement(actions: &ActionState<CameraAction>) -> Result<Vec2> {
    actions
        .axis_pair(CameraAction::Orbit)
        .context("Camera movement is not an axis pair")
        .map(|pair| pair.xy())
}

fn set_yaw_pitch(rig: &mut Rig, camera: &IngameCamera, camera_movement: Vec2, config: &GameConfig) {
    let yaw_pitch = rig.driver_mut::<YawPitch>();
    let yaw = -camera_movement.x * config.camera.mouse_sensitivity_x;
    let pitch = -camera_movement.y * config.camera.mouse_sensitivity_y;
    yaw_pitch.rotate_yaw_pitch(yaw.to_degrees(), pitch.to_degrees());
    let (min_pitch, max_pitch) = get_pitch_extrema(config, camera);
    yaw_pitch.pitch_degrees = yaw_pitch.pitch_degrees.clamp(min_pitch, max_pitch);
}

fn set_look_at(rig: &mut Rig, camera: &IngameCamera) {
    if let Some(look_at) = rig.try_driver_mut::<LookAt>() {
        if let Some(secondary_target) = camera.secondary_target {
            look_at.target = secondary_target.translation
        } else if camera.kind != IngameCameraKind::FirstPerson {
            look_at.target = camera.target.translation
        }
    };
}

fn set_position(rig: &mut Rig, camera: &IngameCamera) {
    let target = if camera.kind != IngameCameraKind::FirstPerson && let Some(secondary_target) = camera.secondary_target {
        secondary_target.translation
    } else {
        camera.target.translation
    };
    rig.driver_mut::<Position>().position = target;
}

fn get_pitch_extrema(config: &GameConfig, camera: &IngameCamera) -> (f32, f32) {
    match camera.kind {
        IngameCameraKind::ThirdPerson => (
            config.camera.third_person.min_pitch,
            config.camera.third_person.max_pitch,
        ),
        IngameCameraKind::FirstPerson => (
            config.camera.first_person.min_pitch,
            config.camera.first_person.max_pitch,
        ),
        _ => unreachable!(),
    }
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
        IngameCameraKind::FirstPerson => (0.0, 0.0),
    };
    camera.desired_distance = (camera.desired_distance - zoom).clamp(min_distance, max_distance);
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
            if let Some(look_at) = rig.try_driver_mut::<LookAt>() {
                look_at.smoothness = config.camera.first_person.tracking_smoothing;
            }
        }
        IngameCameraKind::FixedAngle => {
            rig.driver_mut::<Smooth>().position_smoothness =
                config.camera.fixed_angle.translation_smoothing;
            rig.driver_mut::<Smooth>().rotation_smoothness =
                config.camera.fixed_angle.rotation_smoothing;
        }
    }
}
