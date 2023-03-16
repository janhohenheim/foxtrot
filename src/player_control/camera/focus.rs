use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::CurrentDialog;
use anyhow::Result;
use bevy::prelude::*;

pub fn set_camera_focus(
    mut camera_query: Query<&mut IngameCamera>,
    current_dialog: Option<Res<CurrentDialog>>,
    player_query: Query<&Transform, With<Player>>,
    non_player_query: Query<&GlobalTransform, Without<Player>>,
) -> Result<()> {
    for mut camera in camera_query.iter_mut() {
        if let Some(ref active_dialogue) = current_dialog {
            let global_translation = non_player_query.get(active_dialogue.source)?;
            let translation = global_translation.translation();
            camera.secondary_target = Some(translation);
        } else {
            camera.secondary_target = None;
        }
        for transform in player_query.iter() {
            camera.target = transform.translation;
        }
    }
    Ok(())
}
