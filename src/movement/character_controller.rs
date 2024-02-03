use crate::GameState;
pub(crate) use animations::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_xpbd3d::*;
use bevy_xpbd_3d::PhysicsSet;
pub(crate) use components::*;
pub(crate) use models::*;

mod animations;
mod components;

mod models;

pub(crate) fn general_movement_plugin(app: &mut App) {
    app.add_plugins((TnuaXpbd3dPlugin, TnuaControllerPlugin))
        .register_type::<Jumping>()
        .register_type::<Walking>()
        .register_type::<CharacterAnimations>()
        .add_systems(
            Update,
            (apply_jumping, apply_walking, play_animations)
                .chain()
                .in_set(GeneralMovementSystemSet)
                .before(PhysicsSet::Prepare)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, offset_models_to_controller.after(PhysicsSet::Sync));
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum AnimationState {
    Standing,
    Airborne,
    Walking(f32),
    Running(f32),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct GeneralMovementSystemSet;

pub(crate) fn apply_walking(
    mut character_query: Query<(
        &mut TnuaController,
        &mut Walking,
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

pub(crate) fn apply_jumping(mut character_query: Query<(&mut TnuaController, &mut Jumping)>) {
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
