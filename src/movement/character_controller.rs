use crate::GameState;
pub(crate) use animation::AnimationState;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_xpbd3d::*;
use bevy_xpbd_3d::PhysicsSet;
pub(crate) use components::*;

mod animation;
mod components;
mod models;

/// This plugin communicates with the Tnua character controller by propagating settings found in
/// the control components [`Walk`] and [`Jump`]. It also controls a state machine to determine which animations to play.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((components::plugin, animation::plugin, models::plugin))
        .add_plugins((TnuaXpbd3dPlugin, TnuaControllerPlugin))
        .add_systems(
            Update,
            (apply_jumping, apply_walking)
                .chain()
                .in_set(GeneralMovementSystemSet)
                .before(PhysicsSet::Prepare)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct GeneralMovementSystemSet;

fn apply_walking(
    mut character_query: Query<(
        &mut TnuaController,
        &mut Walk,
        Option<&Sprinting>,
        &FloatHeight,
    )>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("apply_walking").entered();
    for (mut controller, mut walking, sprinting, float_height) in &mut character_query {
        let direction = walking.direction.unwrap_or_default();
        let sprinting_multiplier = sprinting
            .filter(|s| s.requested)
            .map(|s| s.multiplier)
            .unwrap_or(1.);
        let speed = walking.speed * sprinting_multiplier;
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * speed,
            desired_forward: direction.normalize_or_zero(),
            float_height: float_height.0,
            cling_distance: 0.1,
            ..Default::default()
        });
        walking.direction = None;
    }
}

fn apply_jumping(mut character_query: Query<(&mut TnuaController, &mut Jump)>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("apply_jumping").entered();
    for (mut controller, mut jump) in &mut character_query {
        if jump.requested {
            controller.action(TnuaBuiltinJump {
                height: jump.height,
                takeoff_extra_gravity: 10.0,
                ..Default::default()
            });
            jump.requested = false;
        }
    }
}
