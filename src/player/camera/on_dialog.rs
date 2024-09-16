use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, Animator, EaseFunction, Tween};

use crate::dialog::StartDialog;

use super::PlayerCamera;

pub(super) fn plugin(app: &mut App) {
    app.observe(face_dialog_target);
}

fn face_dialog_target(
    trigger: Trigger<StartDialog>,
    q_camera: Query<(Entity, &Transform), With<PlayerCamera>>,
    q_transform: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let target = trigger.entity();
    let Ok(target_transform) = q_transform.get(target).map(|t| t.compute_transform()) else {
        return;
    };
    for (camera, camera_transform) in &q_camera {
        let start = camera_transform.rotation;
        let end = camera_transform
            .looking_at(target_transform.translation, camera_transform.up())
            .rotation;
        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs_f32(1.0),
            TransformRotationLens { start, end },
        );
        commands.entity(camera).insert(Animator::new(tween));
    }
}
