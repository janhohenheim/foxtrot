use crate::movement::general_movement::{Jump, Walker};
use crate::player_control::actions::Actions;
use crate::player_control::camera::MainCamera;
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
                    .with_system(handle_jump.after("set_actions").before("apply_jumping"))
                    .with_system(
                        handle_horizontal_movement
                            .after("set_actions")
                            .before("apply_walking"),
                    )
                    .with_system(set_camera_target.before("follow_target")),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Player;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerSensor;

fn handle_jump(actions: Res<Actions>, mut player_query: Query<&mut Jump, With<Player>>) {
    for mut jump in &mut player_query {
        if actions.jump {
            jump.requested = true;
        }
    }
}

fn handle_horizontal_movement(
    actions: Res<Actions>,
    mut player_query: Query<&mut Walker, With<Player>>,
    camera_query: Query<&MainCamera>,
) {
    let camera = match camera_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    let movement = match actions.player_movement {
        Some(movement) => movement,
        None => return,
    };

    let forward = camera
        .new
        .direction()
        .unwrap_or(Vec3::Z)
        .xz()
        .try_normalize()
        .unwrap_or(Vec2::Y);
    let sideward = forward.perp();
    let forward_action = forward * movement.y;
    let sideward_action = sideward * movement.x;
    let direction = (forward_action + sideward_action).x0y().normalize();

    for mut walker in &mut player_query {
        walker.direction = Some(direction);
        walker.sprinting = actions.sprint;
    }
}

// Todo: Move this into own system -> can focus dialog target
fn set_camera_target(
    mut camera_query: Query<&mut MainCamera>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    for player_transform in player_query.iter() {
        let translation = player_transform.translation();
        for mut camera in &mut camera_query {
            camera.look_at(translation);
        }
    }
}
