use bevy::{app::RunFixedMainLoop, prelude::*};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use leafwing_input_manager::prelude::*;

use super::action::CharacterAction;
use crate::system_set::VariableBeforeFixedGameSet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaAvian3dPlugin::new(RunFixedMainLoop),
        TnuaControllerPlugin::new(RunFixedMainLoop),
    ));
    app.add_systems(
        RunFixedMainLoop,
        (apply_walking, apply_jumping).in_set(VariableBeforeFixedGameSet::CharacterController),
    );
}

fn apply_walking(
    mut character_query: Query<(
        &mut TnuaController,
        &ActionState<CharacterAction>,
        &FloatHeight,
        &WalkSpeed,
    )>,
) {
    for (mut controller, action_state, float_height, max_speed) in &mut character_query {
        let direction = action_state
            .axis_pair(&CharacterAction::Move)
            .normalize_or_zero()
            .extend(0.)
            .xzy();
        let sprinting_factor = if action_state.pressed(&CharacterAction::Sprint) {
            1.5
        } else {
            1.0
        };
        let velocity = direction * max_speed.0 * sprinting_factor;
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: velocity,
            desired_forward: direction,
            float_height: float_height.0,
            cling_distance: 0.1,
            ..Default::default()
        });
    }
}

fn apply_jumping(
    mut character_query: Query<(
        &mut TnuaController,
        &ActionState<CharacterAction>,
        &JumpHeight,
    )>,
) {
    for (mut controller, action_state, jump_height) in &mut character_query {
        if action_state.pressed(&CharacterAction::Jump) {
            controller.action(TnuaBuiltinJump {
                height: jump_height.0,
                takeoff_extra_gravity: 10.0,
                ..Default::default()
            });
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly
pub struct FloatHeight(pub(crate) f32);
#[derive(Debug, Default, Clone, PartialEq, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct WalkSpeed(pub(crate) f32);
#[derive(Debug, Default, Clone, PartialEq, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct JumpHeight(pub(crate) f32);
