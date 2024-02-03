use crate::file_system_interaction::audio::AudioHandles;
use crate::movement::character_controller::*;
use crate::player_control::actions::{DualAxisDataExt, PlayerAction};
use crate::player_control::camera::{CameraUpdateSystemSet, IngameCamera, IngameCameraKind};

use crate::util::trait_extension::Vec3Ext;
use crate::world_interaction::dialog::DialogTarget;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_mod_sysfail::*;
use bevy_tnua::builtins::TnuaBuiltinWalk;
use bevy_tnua::controller::TnuaController;
use bevy_yarnspinner_example_dialogue_view::SpeakerChangeEvent;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

/// This plugin handles everything that has to do with the player's physical representation in the world.
/// This includes movement and rotation that differ from the way the [`MovementPlugin`] already handles characters in general.
pub(crate) fn player_embodiment_plugin(app: &mut App) {
    app.register_type::<Timer>()
        .register_type::<Player>()
        .add_systems(
            Update,
            (
                handle_jump,
                handle_horizontal_movement,
                rotate_to_speaker,
                control_walking_sound,
                handle_camera_kind,
            )
                .chain()
                .before(CameraUpdateSystemSet)
                .before(GeneralMovementSystemSet)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Player;

fn handle_jump(mut player_query: Query<(&ActionState<PlayerAction>, &mut Jump), With<Player>>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_jump").entered();
    for (actions, mut jump) in &mut player_query {
        jump.requested |= actions.pressed(PlayerAction::Jump);
    }
}

#[sysfail(log(level = "error"))]
fn handle_horizontal_movement(
    mut player_query: Query<(&ActionState<PlayerAction>, &mut Walk, &mut Sprinting), With<Player>>,
    camera_query: Query<(&IngameCamera, &Transform), Without<Player>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_horizontal_movement").entered();
    let Some((camera, camera_transform)) = camera_query.iter().next() else {
        return Ok(());
    };

    for (actions, mut walk, mut sprint) in &mut player_query {
        let Some(axis) = actions.axis_pair(PlayerAction::Move) else {
            continue;
        };
        if let Some(movement) = axis.max_normalized() {
            let forward = if camera.kind == IngameCameraKind::FixedAngle {
                camera_transform.up()
            } else {
                camera_transform.forward()
            }
            .horizontal()
            .normalize();

            let sideways = forward.cross(Vec3::Y);
            let forward_action = forward * movement.y;
            let sideways_action = sideways * movement.x;

            let is_looking_backward = forward.dot(forward_action) < 0.0;
            let is_first_person = camera.kind == IngameCameraKind::FirstPerson;
            let modifier = if is_looking_backward && is_first_person {
                0.7
            } else {
                1.
            };
            let direction = forward_action * modifier + sideways_action;

            walk.direction = Some(direction);
            sprint.requested = actions.pressed(PlayerAction::Sprint);
        }
    }
    Ok(())
}

fn handle_camera_kind(
    mut with_player: Query<(&mut Transform, &mut Visibility), With<Player>>,
    camera_query: Query<(&Transform, &IngameCamera), Without<Player>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_camera_kind").entered();
    for (camera_transform, camera) in camera_query.iter() {
        for (mut player_transform, mut visibility) in with_player.iter_mut() {
            match camera.kind {
                IngameCameraKind::FirstPerson => {
                    let horizontal_direction = camera_transform.forward().horizontal();
                    let looking_target = player_transform.translation + horizontal_direction;
                    player_transform.look_at(looking_target, Vec3::Y);
                    *visibility = Visibility::Hidden;
                }
                IngameCameraKind::ThirdPerson | IngameCameraKind::FixedAngle => {
                    *visibility = Visibility::Inherited;
                }
            }
        }
    }
}

fn rotate_to_speaker(
    mut with_player: Query<(&Transform, &mut TnuaController), With<Player>>,
    speakers: Query<(&Transform, &DialogTarget), Without<Player>>,
    mut speaker_change_event: EventReader<SpeakerChangeEvent>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("rotate_to_speaker").entered();
    for speaker_change in speaker_change_event.read() {
        if !speaker_change.speaking {
            continue;
        }

        for (player_transform, mut controller) in with_player.iter_mut() {
            for (speaker_transform, dialog_target) in speakers.iter() {
                if dialog_target.speaker != speaker_change.character_name {
                    continue;
                }
                let direction = speaker_transform.translation - player_transform.translation;
                controller.basis(TnuaBuiltinWalk {
                    desired_forward: direction,
                    ..Default::default()
                });
            }
        }
    }
}

#[sysfail(log(level = "error"))]
fn control_walking_sound(
    time: Res<Time<Virtual>>,
    character_query: Query<&TnuaController, With<Player>>,
    audio: Res<AudioHandles>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("control_walking_sound").entered();
    for controller in character_query.iter() {
        let audio_instance = audio_instances
            .get_mut(&audio.walking)
            .context("Failed to get audio instance from handle")?;
        let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
            continue;
        };
        let has_horizontal_movement = !basis_state.running_velocity.horizontal().is_approx_zero();
        let is_moving_on_ground = has_horizontal_movement && !controller.is_airborne()?;
        if is_moving_on_ground && !time.is_paused() {
            audio_instance.resume(default());
        } else {
            audio_instance.pause(default());
        }
    }
    Ok(())
}
