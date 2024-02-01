use anyhow::{Context, Result};
use bevy::prelude::*;
use std::time::Duration;
mod animations;
mod components;

use crate::file_system_interaction::config::GameConfig;
use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::{TransformExt, Vec3Ext};
use crate::GameState;
pub(crate) use animations::*;
use bevy_mod_sysfail::*;
use bevy_tnua::prelude::*;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective};
use bevy_tnua_xpbd3d::*;
use bevy_xpbd_3d::{prelude as xpbd, prelude::*};
pub(crate) use components::*;

pub(crate) fn general_movement_plugin(app: &mut App) {
    app.add_plugins((TnuaXpbd3dPlugin, TnuaControllerPlugin))
        .register_type::<Jumping>()
        .register_type::<Walking>()
        .register_type::<CharacterAnimations>()
        .add_systems(
            Update,
            (apply_jumping, apply_walking, play_animations, sync_models)
                .chain()
                .in_set(GeneralMovementSystemSet)
                .run_if(in_state(GameState::Playing)),
        );
}

pub(crate) enum AnimationState {
    Standing,
    Airborne,
    Running(f32),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct GeneralMovementSystemSet;

pub(crate) fn apply_jumping(mut character_query: Query<(&mut TnuaController, &Jumping)>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("apply_jumping").entered();
    for (mut controller, jump) in &mut character_query {
        if jump.requested {
            // The jump action must be fed as long as the player holds the button.
            controller.action(TnuaBuiltinJump {
                // The full height of the jump, if the player does not release the button:
                height: 4.0,

                // TnuaBuiltinJump too has other fields that can be configured:
                ..Default::default()
            });
        }
    }
}

pub(crate) fn apply_walking(mut character_query: Query<(&mut TnuaController, &Walking)>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("apply_walking").entered();
    for (mut controller, walking) in &mut character_query {
        controller.basis(TnuaBuiltinWalk {
            // Move in the direction the player entered, at a speed of 10.0:
            desired_velocity: walking.direction.unwrap_or_default() * 10.0,

            // Turn the character in the movement direction:
            desired_forward: walking.direction.unwrap_or_default(),

            // Must be larger than the height of the entity's center from the bottom of its
            // collider, or else the character will not float and Tnua will not work properly:
            float_height: 2.0,

            // TnuaBuiltinWalk has many other fields that can be configured:
            ..Default::default()
        });
    }
}

#[sysfail(log(level = "error"))]
fn sync_models(
    time: Res<Time>,
    mut commands: Commands,
    without_model: Query<(&Transform, &Visibility), Without<Model>>,
    mut with_model: Query<(Entity, &mut Transform, &mut Visibility, &Model)>,
    game_config: Res<GameConfig>,
) -> Result<()> {
    let dt = time.delta_seconds();
    for (model_entity, mut model_transform, mut visibility, model) in with_model.iter_mut() {
        if let Ok((target_transform, target_visibility)) = without_model.get(model.target) {
            let smoothness = game_config.characters.model_sync_smoothing;
            let factor = smoothness_to_lerp_factor(smoothness, dt);
            *model_transform = model_transform.lerp(*target_transform, factor);
            *visibility = *target_visibility;
        } else {
            commands.entity(model_entity).despawn_recursive();
        }
    }
    Ok(())
}
