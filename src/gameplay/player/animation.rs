use std::time::Duration;

use bevy::prelude::*;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective};

use crate::{
    gameplay::{animation::AnimationPlayerLink, crosshair::CrosshairState},
    screens::Screen,
};

use super::assets::PlayerAssets;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAnimations>();
    app.add_systems(Update, play_animations.run_if(in_state(Screen::Gameplay)));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct PlayerAnimations {
    idle: AnimationNodeIndex,
    start_idle: AnimationNodeIndex,
}

pub(crate) fn setup_player_animations(
    trigger: Trigger<OnAdd, AnimationPlayerLink>,
    q_anim_player_link: Query<&AnimationPlayerLink>,
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let anim_player = q_anim_player_link.get(trigger.entity()).unwrap().0;

    let (graph, indices) = AnimationGraph::from_clips([
        assets.idle_animation.clone(),
        assets.start_idle_animation.clone(),
    ]);
    let [idle_index, start_idle_index] = indices.as_slice() else {
        unreachable!()
    };
    let graph_handle = graphs.add(graph);

    let animations = PlayerAnimations {
        idle: *idle_index,
        start_idle: *start_idle_index,
    };
    let transitions = AnimationTransitions::new();
    commands.entity(anim_player).insert((
        animations,
        AnimationGraphHandle(graph_handle),
        transitions,
    ));
}

/// Managed by [`play_animations`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PlayerAnimationState {
    None,
    Idle,
}

fn play_animations(
    mut query: Query<(
        &mut TnuaAnimatingState<PlayerAnimationState>,
        &AnimationPlayerLink,
    )>,
    mut q_animation: Query<(
        &PlayerAnimations,
        &mut AnimationPlayer,
        &mut AnimationTransitions,
    )>,
    assets: Res<PlayerAssets>,
    animation_clips: Res<Assets<AnimationClip>>,
    crosshair_state: Single<&CrosshairState>,
) {
    for (mut animating_state, link) in query.iter_mut() {
        let animation_player_entity = link.0;
        let Ok((animations, mut anim_player, mut transitions)) =
            q_animation.get_mut(animation_player_entity)
        else {
            continue;
        };
        match animating_state.update_by_discriminant(
            // we show the player's hands exactly if and only if the crosshair is visible
            if crosshair_state.wants_invisible.is_empty() {
                PlayerAnimationState::Idle
            } else {
                PlayerAnimationState::None
            },
        ) {
            TnuaAnimatingStateDirective::Maintain { .. } => {}
            TnuaAnimatingStateDirective::Alter {
                // We don't need the old state here, but it's available for transition
                // animations.
                old_state: _,
                state,
            } => match state {
                PlayerAnimationState::None => {
                    transitions
                        .play(
                            &mut anim_player,
                            animations.start_idle,
                            Duration::from_millis(100),
                        )
                        .set_speed(-1.0);
                }
                PlayerAnimationState::Idle => {
                    let start_idle_animation =
                        animation_clips.get(&assets.start_idle_animation).unwrap();
                    const SPEEDUP: f32 = 2.0;
                    let start_idle_duration = start_idle_animation.duration() / SPEEDUP;
                    transitions
                        .play(
                            &mut anim_player,
                            animations.start_idle,
                            Duration::from_millis(0),
                        )
                        .set_speed(SPEEDUP);
                    transitions
                        .play(
                            &mut anim_player,
                            animations.idle,
                            Duration::from_secs_f32(start_idle_duration),
                        )
                        .repeat();
                }
            },
        }
    }
}
