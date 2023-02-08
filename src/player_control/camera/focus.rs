use crate::player_control::camera::MainCamera;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::CurrentDialog;
use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterController;

pub fn set_camera_focus(
    mut camera_query: Query<&mut MainCamera>,
    current_dialog: Option<Res<CurrentDialog>>,
    player_query: Query<(&GlobalTransform, &KinematicCharacterController), With<Player>>,
    non_player_query: Query<
        (&GlobalTransform, Option<&KinematicCharacterController>),
        Without<Player>,
    >,
) {
    for mut camera in camera_query.iter_mut() {
        if let Some(ref active_dialogue) = current_dialog &&
            let Some(target_entity) = active_dialogue.source {
            let (global_translation, kinematic_character_controller) = non_player_query
                .get(target_entity)
                .unwrap();
            let translation = global_translation.translation();
            let up = kinematic_character_controller.map(|kcc| kcc.up).unwrap_or(Vec3::Y);
            camera.set_target(translation);
            camera.set_up(up);
        } else {
            for (global_translation, kinematic_character_controller ) in player_query.iter() {
                let translation = global_translation.translation();
                camera.set_target(translation);
                camera.set_up(kinematic_character_controller.up);
            }
        }
    }
}
