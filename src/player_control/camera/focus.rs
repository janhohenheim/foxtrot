use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::CurrentDialog;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

#[sysfail(log(level = "error"))]
pub(crate) fn set_camera_focus(
    mut camera_query: Query<&mut IngameCamera>,
    current_dialog: Option<Res<CurrentDialog>>,
    player_query: Query<&Transform, With<Player>>,
    non_player_query: Query<&GlobalTransform, Without<Player>>,
) -> Result<()> {
    for mut camera in camera_query.iter_mut() {
        for player_transform in player_query.iter() {
            if let Some(ref active_dialogue) = current_dialog {
                let dialog_target_transform = non_player_query
                    .get(active_dialogue.source)?
                    .compute_transform();
                camera.secondary_target = Some(dialog_target_transform);
            } else {
                camera.secondary_target = None;
            }
            camera.target = *player_transform;
        }
    }
    Ok(())
}
