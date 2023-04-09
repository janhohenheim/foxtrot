use crate::file_system_interaction::audio::AudioHandles;
use crate::file_system_interaction::config::GameConfig;
use crate::movement::general_movement::{GeneralMovementSystemSet, Grounded, Jumping, Walking};
use crate::player_control::actions::{DualAxisDataExt, PlayerAction};
use crate::player_control::camera::{CameraUpdateSystemSet, IngameCamera, IngameCameraKind};
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::{F32Ext, TransformExt, Vec3Ext};
use crate::world_interaction::dialog::CurrentDialog;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;

/// This plugin handles everything that has to do with the player's physical representation in the world.
/// This includes movement and rotation that differ from the way the [`MovementPlugin`] already handles characters in general.
pub(crate) fn player_embodiment_plugin(app: &mut App) {
    app.register_type::<Timer>()
        .register_type::<Player>()
        .add_systems(
            (
                handle_jump,
                handle_horizontal_movement,
                handle_speed_effects,
                rotate_to_speaker.run_if(resource_exists::<CurrentDialog>()),
                control_walking_sound,
                handle_camera_kind,
            )
                .chain()
                .after(CameraUpdateSystemSet)
                .before(GeneralMovementSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Player;

fn handle_jump(mut player_query: Query<(&ActionState<PlayerAction>, &mut Jumping), With<Player>>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_jump").entered();
    for (actions, mut jump) in &mut player_query {
        jump.requested |= actions.pressed(PlayerAction::Jump);
    }
}

#[sysfail(log(level = "error"))]
fn handle_horizontal_movement(
    mut player_query: Query<(&ActionState<PlayerAction>, &mut Walking, &Transform), With<Player>>,
    camera_query: Query<(&IngameCamera, &Transform), Without<Player>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_horizontal_movement").entered();
    let Some((camera, camera_transform)) = camera_query.iter().next() else {
        return Ok(());
    };

    for (actions, mut walk, player_transform) in &mut player_query {
        if let Some(movement) = actions
            .axis_pair(PlayerAction::Move)
            .context("Player movement is not an axis pair")?
            .max_normalized()
        {
            let up = player_transform.up();
            let forward = if camera.kind == IngameCameraKind::FixedAngle {
                camera_transform.up()
            } else {
                camera_transform.forward()
            }
            .split(up)
            .horizontal
            .normalize();

            let sideways = forward.cross(up);
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
            walk.sprinting = actions.pressed(PlayerAction::Sprint);
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
                    let up = player_transform.up();
                    let horizontal_direction = camera_transform.forward().split(up).horizontal;
                    let looking_target = player_transform.translation + horizontal_direction;
                    player_transform.look_at(looking_target, up);
                    *visibility = Visibility::Hidden;
                }
                IngameCameraKind::ThirdPerson | IngameCameraKind::FixedAngle => {
                    *visibility = Visibility::Inherited;
                }
            }
        }
    }
}

fn handle_speed_effects(
    velocities: Query<&Velocity, With<Player>>,
    mut projections: Query<&mut Projection, With<IngameCamera>>,
    config: Res<GameConfig>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_speed_effects").entered();
    for velocity in velocities.iter() {
        let speed_squared = velocity.linvel.length_squared();
        for mut projection in projections.iter_mut() {
            if let Projection::Perspective(ref mut perspective) = projection.deref_mut() {
                let fov_saturation_speed = config.player.fov_saturation_speed;
                let min_fov = config.player.min_fov;
                let max_fov = config.player.max_fov;
                let scale = (speed_squared / fov_saturation_speed.squared())
                    .min(1.0)
                    .squared();
                perspective.fov = min_fov + (max_fov - min_fov) * scale;
            }
        }
    }
}

fn rotate_to_speaker(
    time: Res<Time>,
    mut with_player: Query<(&mut Transform, &Velocity), With<Player>>,
    without_player: Query<&Transform, Without<Player>>,
    current_dialog: Res<CurrentDialog>,
    config: Res<GameConfig>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("rotate_to_speaker").entered();
    let Ok(speaker_transform) = without_player.get(current_dialog.source) else {
         return;
    };
    let dt = time.delta_seconds();

    for (mut transform, velocity) in with_player.iter_mut() {
        let horizontal_velocity = velocity.linvel.split(transform.up()).horizontal;
        if horizontal_velocity.is_approx_zero() {
            let up = transform.up();
            let target_rotation = transform
                .horizontally_looking_at(speaker_transform.translation, up)
                .rotation;
            let smoothness = config.player.rotate_to_speaker_smoothness;
            let factor = smoothness_to_lerp_factor(smoothness, dt);
            let rotation = transform.rotation.slerp(target_rotation, factor);
            transform.rotation = rotation;
        }
    }
}

#[sysfail(log(level = "error"))]
fn control_walking_sound(
    time: Res<Time>,
    character_query: Query<(&Velocity, &Transform, &Grounded), With<Player>>,
    audio: Res<AudioHandles>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("control_walking_sound").entered();
    for (velocity, transform, grounded) in character_query.iter() {
        let audio_instance = audio_instances
            .get_mut(&audio.walking)
            .context("Failed to get audio instance from handle")?;
        let has_horizontal_movement = !velocity
            .linvel
            .split(transform.up())
            .horizontal
            .is_approx_zero();
        let is_moving_on_ground = has_horizontal_movement && grounded.0;
        if is_moving_on_ground && !time.is_paused() {
            audio_instance.resume(default());
        } else {
            audio_instance.pause(default());
        }
    }
    Ok(())
}
