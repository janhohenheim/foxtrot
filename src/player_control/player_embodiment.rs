use crate::movement::general_movement::{Grounded, Jump, JumpState, Velocity, Walker};
use crate::player_control::actions::Actions;
use crate::player_control::camera::PlayerCamera;
use crate::util::trait_extension::Vec2Ext;
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct PlayerEmbodimentPlugin;

/// This plugin handles player related stuff like general_movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerEmbodimentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Timer>()
            .register_type::<Player>()
            .register_type::<PlayerSensor>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_jump.after("apply_gravity").before("apply_force"))
                    .with_system(
                        handle_horizontal_movement
                            .after("set_actions")
                            .before("apply_walking"),
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
    mut player_query: Query<(&Grounded, &mut Velocity, &mut Jump), With<Player>>,
) {
    let dt = time.delta_seconds();
    let jump_requested = actions.jump;
    for (grounded, mut velocity, mut jump) in &mut player_query {
        if jump_requested && grounded.is_grounded() {
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
            velocity.0.y += jump.speed_fraction() * jump.speed * dt
        }
    }
}

fn handle_horizontal_movement(
    actions: Res<Actions>,
    mut player_query: Query<&mut Walker, With<Player>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
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
    let movement = (forward_action + sideward_action).x0y().normalize();

    for mut walker in &mut player_query {
        walker.direction = Some(movement);
    }
}
