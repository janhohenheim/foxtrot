use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use leafwing_input_manager::prelude::*;

use super::action::CharacterAction;
use crate::system_set::FixedGameSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(WalkControllerConfig, JumpControllerConfig)>();
    app.add_plugins((
        TnuaAvian3dPlugin::new(PhysicsSchedule),
        TnuaControllerPlugin::new(PhysicsSchedule),
    ));
    app.add_systems(
        FixedUpdate,
        (apply_walking, apply_jumping).in_set(FixedGameSet::CharacterController),
    );
}

fn apply_walking(
    mut character_query: Query<(
        &mut TnuaController,
        &ActionState<CharacterAction>,
        &WalkControllerConfig,
    )>,
) {
    for (mut controller, action_state, walk) in &mut character_query {
        let axis = action_state
            .axis_pair(&CharacterAction::Move)
            .normalize_or_zero();
        let direction = Vec3 {
            x: axis.x,
            y: 0.0,
            z: -axis.y,
        };
        let sprinting_factor = if action_state.pressed(&CharacterAction::Sprint) {
            walk.sprint_multiplier
        } else {
            1.0
        };
        let velocity = direction * walk.max_speed * sprinting_factor;
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: velocity,
            desired_forward: direction,
            float_height: walk.float_height,
            cling_distance: 0.1,
            ..Default::default()
        });
    }
}

fn apply_jumping(
    mut character_query: Query<(
        &mut TnuaController,
        &ActionState<CharacterAction>,
        &JumpControllerConfig,
    )>,
) {
    for (mut controller, action_state, jump) in &mut character_query {
        if action_state.pressed(&CharacterAction::Jump) {
            controller.action(TnuaBuiltinJump {
                height: jump.height,
                takeoff_extra_gravity: 10.0,
                ..Default::default()
            });
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Component)]
struct WalkControllerConfig {
    max_speed: f32,
    sprint_multiplier: f32,
    /// Must be larger than the height of the entity's center from the bottom of its
    /// collider, or else the character will not float and Tnua will not work properly
    float_height: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Component)]
struct JumpControllerConfig {
    height: f32,
}
