use std::{iter, time::Duration};

use bevy::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, Animator, EaseFunction, Tween};

use crate::dialog::StartDialog;

use super::PlayerCamera;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DialogCameraTarget>();
    app.observe(face_dialog_target);
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct DialogCameraTarget;

fn face_dialog_target(
    trigger: Trigger<StartDialog>,
    q_camera: Query<(Entity, &Transform), With<PlayerCamera>>,
    q_transform: Query<&GlobalTransform>,
    q_camera_target: Query<(), With<DialogCameraTarget>>,
    q_children: Query<&Children>,
    mut commands: Commands,
) {
    let target = trigger.entity();
    // Check if there is a descendant with the DialogCameraTarget component.
    // If yes, let's use that one as our camera's target to look at.
    // If no, we fallback to the trigger entity itself, i.e. the rigid body associated with this dialog.
    let camera_target = iter::once(target)
        .chain(q_children.iter_descendants(target))
        .find(|e| q_camera_target.contains(*e))
        .unwrap_or(target);
    let Ok(target_transform) = q_transform
        .get(camera_target)
        .map(|t| t.compute_transform())
    else {
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
