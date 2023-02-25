use crate::file_system_interaction::audio::AudioHandles;
use crate::movement::general_movement::{
    apply_jumping, apply_walking, reset_movement_components, Grounded, Jumping, Walking,
};
use crate::player_control::actions::{DualAxisDataExt, PlayerAction};
use crate::player_control::camera::{
    focus::switch_kind as switch_camera_kind, IngameCamera, IngameCameraKind,
    UpdateCameraTransformLabel,
};
use crate::util::log_error::log_errors;
use crate::util::trait_extension::{F32Ext, TransformExt, Vec3Ext};
use crate::world_interaction::dialog::CurrentDialog;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;

pub struct PlayerEmbodimentPlugin;

/// This plugin handles everything that has to do with the player's physical representation in the world.
/// This includes movement and rotation that differ from the way the [`MovementPlugin`] already handles characters in general.
impl Plugin for PlayerEmbodimentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Timer>()
            .register_type::<Player>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(
                        handle_jump
                            .after(reset_movement_components)
                            .before(apply_jumping),
                    )
                    .with_system(
                        handle_horizontal_movement
                            .pipe(log_errors)
                            .after(UpdateCameraTransformLabel)
                            .after(reset_movement_components)
                            .before(apply_walking),
                    )
                    .with_system(
                        handle_camera_kind
                            .after(switch_camera_kind)
                            .before(apply_walking),
                    )
                    .with_system(handle_speed_effects)
                    .with_system(rotate_to_speaker)
                    .with_system(control_walking_sound.pipe(log_errors)),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Player;

fn handle_jump(mut player_query: Query<(&ActionState<PlayerAction>, &mut Jumping), With<Player>>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_jump").entered();
    for (actions, mut jump) in &mut player_query {
        jump.requested |= actions.pressed(PlayerAction::Jump);
    }
}

fn handle_horizontal_movement(
    mut player_query: Query<(&ActionState<PlayerAction>, &mut Walking, &Transform), With<Player>>,
    camera_query: Query<&IngameCamera>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_horizontal_movement").entered();
    let camera = match camera_query.iter().next() {
        Some(camera) => camera,
        None => return Ok(()),
    };

    for (actions, mut walk, transform) in &mut player_query {
        if let Some(movement) = actions
            .axis_pair(PlayerAction::Move)
            .context("Player movement is not an axis pair")?
            .max_normalized()
        {
            let forward = camera
                .forward()
                .split(transform.up())
                .horizontal
                .normalize();
            let sideways = forward.cross(transform.up());
            let forward_action = forward * movement.y;
            let sideways_action = sideways * movement.x;

            let is_looking_backward = forward.dot(forward_action) < 0.0;
            let is_first_person = matches!(camera.kind, IngameCameraKind::FirstPerson(_));
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
                IngameCameraKind::FirstPerson(_) => {
                    let up = camera.up();
                    let horizontal_direction = camera_transform.forward().split(up).horizontal;
                    let looking_target = player_transform.translation + horizontal_direction;
                    player_transform.look_at(looking_target, up);
                    visibility.is_visible = false;
                }
                IngameCameraKind::ThirdPerson(_) | IngameCameraKind::FixedAngle(_) => {
                    visibility.is_visible = true
                }
            }
        }
    }
}

fn handle_speed_effects(
    velocities: Query<&Velocity, With<Player>>,
    mut projections: Query<&mut Projection, With<IngameCamera>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("handle_speed_effects").entered();
    for velocity in velocities.iter() {
        let speed_squared = velocity.linvel.length_squared();
        for mut projection in projections.iter_mut() {
            if let Projection::Perspective(ref mut perspective) = projection.deref_mut() {
                const MAX_SPEED_FOR_FOV: f32 = 12.;
                const MIN_FOV: f32 = 0.75;
                const MAX_FOV: f32 = 1.5;
                let scale = (speed_squared / MAX_SPEED_FOR_FOV.squared())
                    .min(1.0)
                    .squared();
                perspective.fov = MIN_FOV + (MAX_FOV - MIN_FOV) * scale;
            }
        }
    }
}

fn rotate_to_speaker(
    time: Res<Time>,
    mut with_player: Query<(&mut Transform, &Velocity), With<Player>>,
    without_player: Query<&Transform, Without<Player>>,
    current_dialog: Option<Res<CurrentDialog>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("rotate_to_speaker").entered();
    let speaker_entity = current_dialog
        .map(|current_dialog| current_dialog.source)
        .and_then(|source| without_player.get(source).ok());
    let speaker_transform = match speaker_entity {
        Some(speaker_transform) => speaker_transform,
        None => return,
    };
    let dt = time.delta_seconds();

    for (mut transform, velocity) in with_player.iter_mut() {
        let horizontal_velocity = velocity.linvel.split(transform.up()).horizontal;
        if horizontal_velocity.is_approx_zero() {
            let up = transform.up();
            let target_rotation = transform
                .horizontally_looking_at(speaker_transform.translation, up)
                .rotation;
            const SMOOTHNESS: f32 = 4.;
            let scale = (SMOOTHNESS * dt).min(1.);
            let rotation = transform.rotation.slerp(target_rotation, scale);
            transform.rotation = rotation;
        }
    }
}

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
