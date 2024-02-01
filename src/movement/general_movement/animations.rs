use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::movement::general_movement::{AnimationState, CharacterAnimations};
use bevy::animation::AnimationPlayer;
use bevy::log::info_span;
use bevy::prelude::Query;
use bevy_mod_sysfail::sysfail;
use bevy_tnua::builtins::TnuaBuiltinWalk;
use bevy_tnua::controller::TnuaController;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective};
use std::time::Duration;

#[sysfail(log(level = "error"))]
pub(crate) fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    mut query: Query<(
        &mut TnuaAnimatingState<AnimationState>,
        &TnuaController,
        &AnimationEntityLink,
        &CharacterAnimations,
    )>,
) -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("play_animations").entered();
    for (mut animating_state, controller, animation_entity_link, animations) in query.iter_mut() {
        let mut animation_player = animation_player.get_mut(animation_entity_link.0)?;

        match animating_state.update_by_discriminant({
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                continue;
            };
            let speed = basis_state.running_velocity.length();
            if controller.is_airborne()? {
                AnimationState::Airborne
            } else if 0.01 < speed {
                AnimationState::Running(speed)
            } else {
                AnimationState::Standing
            }
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => {
                if let AnimationState::Running(speed) = state {
                    animation_player.set_speed(*speed);
                }
            }
            TnuaAnimatingStateDirective::Alter {
                // We don't need the old state here, but it's available for transition
                // animations.
                old_state: _,
                state,
            } => match state {
                AnimationState::Airborne => {
                    animation_player
                        .play_with_transition(
                            animations.aerial.clone_weak(),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
                AnimationState::Standing => {
                    animation_player
                        .play_with_transition(
                            animations.idle.clone_weak(),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
                AnimationState::Running(speed) => {
                    animation_player
                        .play_with_transition(
                            animations.walk.clone_weak(),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
            },
        }
    }
    Ok(())
}
