use crate::player_control::camera::MainCamera;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::CurrentDialog;
use bevy::prelude::*;

pub fn set_camera_focus(
    mut camera_query: Query<&mut MainCamera>,
    current_dialog: Option<Res<CurrentDialog>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    non_player_query: Query<&GlobalTransform, Without<Player>>,
) {
    for mut camera in camera_query.iter_mut() {
        if let Some(ref active_dialogue) = current_dialog &&
            let Some(target_entity) = active_dialogue.source {
            let translation = non_player_query
                .get(target_entity)
                .unwrap()
                .translation();
            camera.set_target(translation);
        } else {
            for player_transform in player_query.iter() {
                let translation = player_transform.translation();
                camera.set_target(translation);
            }
        }
    }
}
