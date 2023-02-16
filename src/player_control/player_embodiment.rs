use crate::file_system_interaction::audio::AudioHandles;
use crate::movement::general_movement::{
    apply_force, apply_jumping, apply_walking, reset_movement_components, Grounded, Jumping,
    Velocity, Walking,
};
use crate::player_control::actions::{set_actions, Actions};
use crate::player_control::camera::{
    focus::switch_kind as switch_camera_kind, update_transform as update_camera_transform,
    IngameCamera, IngameCameraKind,
};
use crate::util::trait_extension::{F32Ext, TransformExt, Vec2Ext, Vec3Ext};
use crate::world_interaction::dialog::CurrentDialog;
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_rapier3d::prelude::*;
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
                    .with_system(handle_jump.after(set_actions).before(apply_jumping))
                    .with_system(
                        handle_horizontal_movement
                            .after(set_actions)
                            .after(update_camera_transform)
                            .before(apply_walking),
                    )
                    .with_system(
                        set_camera_actions
                            .after(set_actions)
                            .before(update_camera_transform)
                            .before(apply_walking),
                    )
                    .with_system(
                        handle_camera_kind
                            .after(switch_camera_kind)
                            .before(apply_walking),
                    )
                    .with_system(
                        handle_speed_effects
                            .after(apply_force)
                            .before(reset_movement_components),
                    )
                    .with_system(
                        rotate_to_speaker
                            .after(apply_force)
                            .before(reset_movement_components),
                    )
                    .with_system(control_walking_sound.after(set_actions)),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Player;

fn handle_jump(actions: Res<Actions>, mut player_query: Query<&mut Jumping, With<Player>>) {
    for mut jump in &mut player_query {
        if actions.player.jump {
            jump.requested = true;
        }
    }
}

fn handle_horizontal_movement(
    actions: Res<Actions>,
    mut player_query: Query<&mut Walking, With<Player>>,
    camera_query: Query<&IngameCamera>,
) {
    let camera = match camera_query.iter().next() {
        Some(camera) => camera,
        None => return,
    };
    let movement = match actions.player.movement {
        Some(movement) => movement,
        None => return,
    };

    let forward = camera.forward().xz().normalize();
    let sideward = forward.perp();
    let forward_action = forward * movement.y;
    let sideward_action = sideward * movement.x;
    let direction = (forward_action + sideward_action).x0y().normalize();

    for mut walk in &mut player_query {
        walk.direction = Some(direction);
        walk.sprinting = actions.player.sprint;
    }
}

pub fn set_camera_actions(actions: Res<Actions>, mut camera_query: Query<&mut IngameCamera>) {
    let mut camera = match camera_query.iter_mut().next() {
        Some(camera) => camera,
        None => return,
    };

    camera.actions = actions.camera.clone();
}

fn handle_camera_kind(
    mut with_player: Query<(&mut Transform, &mut Visibility), With<Player>>,
    camera_query: Query<(&Transform, &IngameCamera), Without<Player>>,
) {
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
    for velocity in velocities.iter() {
        let speed_squared = velocity.0.length_squared();
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
    let speaker_entity = current_dialog
        .map(|current_dialog| current_dialog.source)
        .and_then(|source| without_player.get(source).ok());
    let speaker_transform = match speaker_entity {
        Some(speaker_transform) => speaker_transform,
        None => return,
    };
    let dt = time.delta_seconds();

    for (mut transform, velocity) in with_player.iter_mut() {
        let horizontal_velocity = velocity.0.split(transform.up()).horizontal;
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
    character_query: Query<
        (
            &KinematicCharacterControllerOutput,
            &KinematicCharacterController,
            &Grounded,
        ),
        With<Player>,
    >,
    audio: Res<AudioHandles>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for (output, controller, grounded) in character_query.iter() {
        if let Some(instance) = audio_instances.get_mut(&audio.walking) {
            let has_horizontal_movement = !output
                .effective_translation
                .split(controller.up)
                .horizontal
                .is_approx_zero();
            let is_moving_on_ground = has_horizontal_movement && grounded.is_grounded();
            if is_moving_on_ground {
                instance.resume(default());
            } else {
                instance.pause(default());
            }
        }
    }
}
