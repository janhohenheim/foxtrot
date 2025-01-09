use crate::system_set::GameSystemSet;
use crate::util::error;

use bevy::{animation::AnimationPlayer, prelude::*};
use blenvy::blueprints::animation::{BlueprintAnimationPlayerLink, BlueprintAnimations};
use bevy_tnua::{
    builtins::TnuaBuiltinWalk, controller::TnuaController, TnuaAnimatingState,
    TnuaAnimatingStateDirective,
};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CharacterAnimationNames>().add_systems(
        Update,
        play_animations
            .pipe(error)
            .in_set(GameSystemSet::PlayAnimation),
    );
}

/// Managed by [`play_animations`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum AnimationState {
    Standing,
    Airborne,
    Walking(f32),
    Running(f32),
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
struct CharacterAnimationNames {
    idle: String,
    walk: String,
    aerial: String,
}

fn play_animations(
    mut query: Query<(
        Entity,
        &mut TnuaAnimatingState<AnimationState>,
        &TnuaController,
        &BlueprintAnimationPlayerLink,
        &BlueprintAnimations,
    )>,
    children: Query<&Children>,
    animation_names: Query<&CharacterAnimationNames>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("play_animations").entered();
    for (entity, mut animating_state, controller, link, animations) in query.iter_mut() {
        let Some(animation_names) = children
            .iter_descendants(entity)
            .filter_map(|entity| animation_names.get(entity).ok())
            .next()
        else {
            continue;
        };
        let (mut animation_player, mut animation_transitions) =
            animation_players.get_mut(link.0)?;
        match animating_state.update_by_discriminant({
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                continue;
            };
            let speed = basis_state.running_velocity.length();
            if controller.is_airborne()? {
                AnimationState::Airborne
            } else if speed > 10.0 {
                AnimationState::Running(speed)
            } else if speed > 0.01 {
                AnimationState::Walking(speed)
            } else {
                AnimationState::Standing
            }
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => {
                if let AnimationState::Running(speed) = state {
                    let anim_speed = (speed / 7.0).max(1.0);
                    let factor = anim_speed / speed;
                    animation_player.adjust_speeds(factor);
                }
            }
            TnuaAnimatingStateDirective::Alter {
                // We don't need the old state here, but it's available for transition
                // animations.
                old_state: _,
                state,
            } => match state {
                AnimationState::Airborne | AnimationState::Running(..) => {
                    animation_transitions
                        .play(
                            &mut animation_player,
                            *animations
                                .named_indices
                                .get(&animation_names.aerial)
                                .expect("Aerial animation should be in animation list"),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
                AnimationState::Standing => {
                    animation_transitions
                        .play(
                            &mut animation_player,
                            *animations
                                .named_indices
                                .get(&animation_names.idle)
                                .expect("Idle animation should be in animation list"),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
                AnimationState::Walking(_speed) => {
                    animation_transitions
                        .play(
                            &mut animation_player,
                            *animations
                                .named_indices
                                .get(&animation_names.walk)
                                .expect("Walk animation should be in animation list"),
                            Duration::from_secs_f32(0.1),
                        )
                        .repeat();
                }
            },
        }
    }
    Ok(())
}
