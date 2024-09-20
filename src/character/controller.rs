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
        Entity,
        &mut TnuaController,
        &ActionState<CharacterAction>,
        &WalkControllerConfig,
        Option<&OverrideForwardDirection>,
        Has<ControllerDisabled>,
    )>,
    forward_reference_query: Query<&Transform>,
) {
    for (entity, mut controller, action_state, walk, forward_reference, is_disabled) in
        &mut character_query
    {
        let forward_reference_entity = forward_reference.map_or(entity, |e| e.0);
        let Ok(forward_reference) = forward_reference_query.get(forward_reference_entity) else {
            error!("Forward reference entity not found");
            continue;
        };
        let axis = action_state.axis_pair(&CharacterAction::Move);

        let (forward, right) = {
            let forward = forward_reference.forward().as_vec3();
            let up = forward_reference.up().as_vec3();

            let vertical = up * forward.dot(up);
            let horizontal_forward = (forward - vertical).normalize();
            let horizontal_right = horizontal_forward.cross(up);
            (horizontal_forward, horizontal_right)
        };
        let direction = forward * axis.y + right * axis.x;

        let speed_factor = if is_disabled {
            0.0
        } else if action_state.pressed(&CharacterAction::Sprint) {
            walk.sprint_multiplier
        } else {
            1.0
        };

        let velocity = direction * walk.max_speed * speed_factor;
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
    mut character_query: Query<
        (
            &mut TnuaController,
            &ActionState<CharacterAction>,
            &JumpControllerConfig,
        ),
        Without<ControllerDisabled>,
    >,
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
pub struct ControllerDisabled;

/// The entity who's forward direction on the horizontal place will be
/// used to determine the character's forward direction for the [`WalkControllerConfig`].
/// Used to make the player character always walk in the direction the camera is facing.
/// Todo: make this an optional member on `WalkControllerConfig` once Blenvy supports relations.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub(crate) struct OverrideForwardDirection(pub(crate) Entity);

#[derive(Debug, Default, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Component)]
struct JumpControllerConfig {
    height: f32,
}
