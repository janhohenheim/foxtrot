use crate::actions::Actions;
use crate::camera::PlayerCamera;
use crate::loading::AnimationAssets;
use crate::movement::{CharacterVelocity, Grounded, Jump, JumpState};
use crate::spawning::AnimationEntityLink;
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Timer>()
            .register_type::<Player>()
            .register_type::<PlayerSensor>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_jump.after("apply_gravity").before("apply_velocity"))
                    .with_system(
                        handle_horizontal_movement
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(
                        play_animations
                            .label("play_animations")
                            .after("apply_velocity")
                            .before("reset_velocity"),
                    ),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Player;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerSensor;

fn handle_jump(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Grounded, &mut CharacterVelocity, &mut Jump), With<Player>>,
) {
    let dt = time.delta_seconds();
    let jump_requested = actions.jump;
    for (grounded, mut velocity, mut jump) in &mut player_query {
        let y_speed = 10.;
        if jump_requested && f32::from(grounded.time_since_last_grounded) < 0.00001 {
            jump.time_since_start.start();
            jump.state = JumpState::InProgress;
        } else {
            jump.time_since_start.update(dt);

            let jump_ended = f32::from(jump.time_since_start) >= jump.duration;
            if jump_ended {
                jump.state = JumpState::Done;
            }
        }
        if matches!(jump.state, JumpState::InProgress) {
            velocity.0.y += jump.speed_fraction() * y_speed * dt
        }
    }
}

fn handle_horizontal_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut CharacterVelocity,), With<Player>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    let dt = time.delta_seconds();
    let speed = 6.0;

    let camera = match camera_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    let actions = match actions.player_movement {
        Some(actions) => actions,
        None => return,
    };

    let forward = (-camera.translation)
        .xz()
        .try_normalize()
        .unwrap_or(Vec2::Y);
    let sideward = forward.perp();
    let forward_action = forward * actions.y;
    let sideward_action = sideward * actions.x;
    let movement = (forward_action + sideward_action).normalize() * speed * dt;

    for (mut velocity,) in &mut player_query {
        velocity.0.x += movement.x;
        velocity.0.z += movement.y;
    }
}

fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    player_query: Query<(&CharacterVelocity, &Grounded, &AnimationEntityLink)>,
    mut model_query: Query<&mut Transform>,
    animations: Res<AnimationAssets>,
) {
    for (velocity, grounded, animation_entity_link) in player_query.iter() {
        let mut animation_player = animation_player
            .get_mut(animation_entity_link.0)
            .expect("animation_entity_link held entity without animation player");

        let horizontal_velocity = Vec3 {
            y: 0.,
            ..velocity.0
        };
        let is_in_air = f32::from(grounded.time_since_last_grounded) > 1e-4;
        let has_horizontal_movement = horizontal_velocity.length() > 1e-4;

        if is_in_air {
            animation_player
                .play(animations.character_running.clone_weak())
                .repeat();
        } else if has_horizontal_movement {
            animation_player
                .play(animations.character_walking.clone_weak())
                .repeat();
        } else {
            animation_player
                .play(animations.character_idle.clone_weak())
                .repeat();
        }

        if has_horizontal_movement {
            model_query
                .get_mut(animation_entity_link.0)
                .expect("animation_entity_link held entity without transform")
                .look_at(horizontal_velocity, Vect::Y);
        }
    }
}
