use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::util::trait_extension::{F32Ext, Vec2Ext};
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn update_rig(
    mut camera_query: Query<(&mut IngameCamera, &mut Rig, &ActionState<CameraAction>)>,
    rapier_context: Res<RapierContext>,
    config: Res<GameConfig>,
) -> Result<()> {
    for (mut camera, mut rig, actions) in camera_query
        .iter_mut()
        .filter(|query_result| query_result.0.kind == IngameCameraKind::ThirdPerson)
    {
        if let Some(secondary_target) = camera.secondary_target {
            rig.driver_mut::<LookAt>().target = secondary_target;
            rig.driver_mut::<Position>().position = secondary_target;
        } else {
            rig.driver_mut::<LookAt>().target = camera.target;
            rig.driver_mut::<Position>().position = camera.target;
        }

        let camera_movement = actions
            .axis_pair(CameraAction::Pan)
            .context("Camera movement is not an axis pair")?
            .xy();
        if !camera_movement.is_approx_zero() {
            let yaw = -camera_movement.x * config.camera.mouse_sensitivity_x;
            let pitch = -camera_movement.y * config.camera.mouse_sensitivity_y;
            let yaw_pitch = rig.driver_mut::<YawPitch>();
            yaw_pitch.rotate_yaw_pitch(yaw.to_degrees(), pitch.to_degrees());
        }

        let zoom =
            actions.clamped_value(CameraAction::Zoom) * config.camera.third_person.zoom_speed;

        camera.desired_distance = (camera.desired_distance - zoom).clamp(
            config.camera.third_person.min_distance,
            config.camera.third_person.max_distance,
        );
        let current_offset = rig.driver::<Arm>().offset;
        let origin = camera.target;
        let direction = current_offset.normalize();

        let max_toi = camera.desired_distance;
        let solid = true;
        let mut filter = QueryFilter::only_fixed();
        filter.flags |= QueryFilterFlags::EXCLUDE_SENSORS;

        let min_distance_to_objects = config.camera.third_person.min_distance_to_objects;
        let distance = rapier_context
            .cast_ray(origin, direction, max_toi, solid, filter)
            .map(|(_entity, toi)| toi - min_distance_to_objects)
            .unwrap_or(max_toi);

        let offset = direction * distance;

        let original_distance_squared = current_offset.length_squared();
        let translation_smoothing = if distance.squared() < original_distance_squared - 1e-3 {
            config
                .camera
                .third_person
                .translation_smoothing_going_closer
        } else {
            config
                .camera
                .third_person
                .translation_smoothing_going_further
        };
        rig.driver_mut::<Arm>().offset = offset;
        //rig.driver_mut::<Smooth>().position_smoothness = translation_smoothing;
    }
    Ok(())
}
