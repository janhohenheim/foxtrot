use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::CurrentDialog;
use anyhow::Result;
use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterController;

pub fn set_camera_focus(
    mut camera_query: Query<&mut IngameCamera>,
    current_dialog: Option<Res<CurrentDialog>>,
    player_query: Query<(&GlobalTransform, &KinematicCharacterController), With<Player>>,
    non_player_query: Query<&GlobalTransform, Without<Player>>,
) -> Result<()> {
    for mut camera in camera_query.iter_mut() {
        if let Some(ref active_dialogue) = current_dialog {
            let global_translation = non_player_query.get(active_dialogue.source)?;
            let translation = global_translation.translation();
            *camera.secondary_target_mut() = Some(translation);
        } else {
            *camera.secondary_target_mut() = None;
        }
        for (global_translation, kinematic_character_controller) in player_query.iter() {
            let translation = global_translation.translation();
            camera.set_primary_target(translation);
            *camera.up_mut() = kinematic_character_controller.up;
        }
    }
    Ok(())
}

pub fn switch_kind(mut camera: Query<&mut IngameCamera>) {
    const THIRD_TO_FIRST_PERSON_ZOOM_THRESHOLD: f32 = 1.;
    const THIRD_PERSON_TO_FIXED_ANGLE_ZOOM_THRESHOLD: f32 = 9.5;
    for mut camera in camera.iter_mut() {
        if let Some(zoom) = camera.actions.zoom {
            let new_kind = match &camera.kind {
                IngameCameraKind::ThirdPerson(third_person)
                    if zoom > 1e-5
                        && third_person.distance < THIRD_TO_FIRST_PERSON_ZOOM_THRESHOLD =>
                {
                    Some(IngameCameraKind::FirstPerson(third_person.into()))
                }
                IngameCameraKind::ThirdPerson(third_person)
                    if zoom < -1e-5
                        && third_person.distance > THIRD_PERSON_TO_FIXED_ANGLE_ZOOM_THRESHOLD =>
                {
                    Some(IngameCameraKind::FixedAngle(third_person.into()))
                }
                IngameCameraKind::FixedAngle(fixed_angle)
                    if zoom > 1e-5
                        && fixed_angle.distance < THIRD_PERSON_TO_FIXED_ANGLE_ZOOM_THRESHOLD =>
                {
                    Some(IngameCameraKind::ThirdPerson(fixed_angle.into()))
                }
                IngameCameraKind::FirstPerson(first_person) if zoom < -1e-5 => {
                    Some(IngameCameraKind::ThirdPerson(first_person.into()))
                }
                _ => None,
            };
            if let Some(new_kind) = new_kind {
                camera.kind = new_kind;
            }
        }
    }
}
