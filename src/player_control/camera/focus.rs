use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::dialog::DialogTarget;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use bevy_yarn_slinger::prelude::DialogueRunner;
use bevy_yarn_slinger_example_dialogue_view::SpeakerChangeEvent;

#[sysfail(log(level = "error"))]
pub(crate) fn set_camera_focus(
    mut camera_query: Query<&mut IngameCamera>,
    mut speaker_change_events: EventReader<SpeakerChangeEvent>,
    player_query: Query<&Transform, With<Player>>,
    dialog_targets: Query<(&Transform, &DialogTarget), Without<Player>>,
    dialog_runner: Query<&DialogueRunner>,
) -> Result<()> {
    for mut camera in camera_query.iter_mut() {
        for player_transform in player_query.iter() {
            for event in speaker_change_events.read() {
                if event.speaking {
                    for (dialog_target_transform, dialog_target) in dialog_targets.iter() {
                        if dialog_target.speaker == event.character_name {
                            camera.secondary_target = Some(*dialog_target_transform);
                        }
                    }
                } else if !dialog_runner.single().is_running() {
                    camera.secondary_target = None;
                }
            }
            camera.target = *player_transform;
        }
    }
    Ok(())
}
