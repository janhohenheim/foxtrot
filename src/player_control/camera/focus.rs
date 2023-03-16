use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::CurrentDialog;
use anyhow::Result;
use bevy::prelude::*;

pub fn set_camera_focus(
    time: Res<Time>,
    mut camera_query: Query<&mut IngameCamera>,
    current_dialog: Option<Res<CurrentDialog>>,
    player_query: Query<&Transform, With<Player>>,
    non_player_query: Query<&GlobalTransform, Without<Player>>,
) -> Result<()> {
    let dt = time.delta_seconds();
    for mut camera in camera_query.iter_mut() {
        for player_transform in player_query.iter() {
            let target = if let Some(ref active_dialogue) = current_dialog {
                let dialog_target_transform = non_player_query.get(active_dialogue.source)?;
                dialog_target_transform.compute_transform()
            } else {
                *player_transform
            };
            let smoothness = 10.0;
            let factor = (smoothness * dt).min(1.0);
            camera.target.translation = camera.target.translation.lerp(target.translation, factor);
            camera.target.rotation = camera.target.rotation.slerp(target.rotation, factor);
        }
    }
    Ok(())
}
