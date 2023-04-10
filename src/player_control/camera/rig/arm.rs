use crate::file_system_interaction::config::GameConfig;
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::F32Ext;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;

pub(crate) fn get_arm_distance(
    camera: &IngameCamera,
    transform: &Transform,
    rapier_context: &RapierContext,
    config: &GameConfig,
) -> Option<f32> {
    match camera.kind {
        IngameCameraKind::ThirdPerson => Some(get_distance_to_collision(
            rapier_context,
            config,
            camera,
            transform,
        )),
        IngameCameraKind::FixedAngle => Some(camera.desired_distance),
        _ => None,
    }
}

pub(crate) fn get_zoom_smoothness(
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

pub(crate) fn set_arm(rig: &mut Rig, distance: f32, zoom_smoothness: f32, dt: f32) {
    let factor = smoothness_to_lerp_factor(zoom_smoothness, dt);
    let arm_length = &mut rig.driver_mut::<Arm>().offset.z;
    *arm_length = arm_length.lerp(distance, factor);
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
        _ => unreachable!(),
    };

    rapier_context
        .cast_ray_and_get_normal(origin, direction, max_toi, solid, filter)
        .map(|(_entity, ray_intersection)| {
            get_distance_such_that_min_distance_from_collision_is_ensured(
                ray_intersection,
                direction,
                min_distance,
            )
        })
        .unwrap_or(max_toi)
}

fn get_distance_such_that_min_distance_from_collision_is_ensured(
    ray_intersection: RayIntersection,
    direction: Vec3,
    min_distance: f32,
) -> f32 {
    //  Wall
    //  ↑
    //  │╲
    //  │  ╲
    //  │    ╲ Hypotenuse, direction = direction
    //  │      ╲
    //  │        ╲
    //  │      ( α ╲
    //  └─────────────→ Normal, magnitude = min_distance
    //  │              ╲
    //  │                ╲   Target vector. Magnitude = total length (toi) - hypotenuse
    //  │                  ╲
    let adjacent_side = min_distance;
    let angle = direction.angle_between(-ray_intersection.normal);
    let hypotenuse = adjacent_side / angle.cos();
    ray_intersection.toi - hypotenuse
}
