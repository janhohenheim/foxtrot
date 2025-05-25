//! Player animations.

use std::time::Duration;

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective};

use crate::{
    PostPhysicsAppSystems,
    gameplay::{animation::AnimationPlayers, crosshair::CrosshairState},
    screens::Screen,
};

use super::assets::PlayerAssets;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAnimations>();
    app.add_systems(
        Update,
        play_animations
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::PlayAnimations),
    );
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct PlayerAnimations {
    idle: AnimationNodeIndex,
    a_pose: AnimationNodeIndex,
}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn setup_player_animations(
    trigger: Trigger<OnAdd, AnimationPlayers>,
    q_anim_players: Query<&AnimationPlayers>,
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let anim_players = q_anim_players.get(trigger.target()).unwrap();
    for anim_player in anim_players.iter() {
        let (graph, indices) = AnimationGraph::from_clips([
            assets.idle_animation.clone(),
            assets.a_pose_animation.clone(),
        ]);
        let [idle_index, a_pose_index] = indices.as_slice() else {
            unreachable!()
        };
        let graph_handle = graphs.add(graph);

        let animations = PlayerAnimations {
            idle: *idle_index,
            a_pose: *a_pose_index,
        };
        let transitions = AnimationTransitions::new();
        commands.entity(anim_player).insert((
            animations,
            AnimationGraphHandle(graph_handle),
            transitions,
        ));
    }
}

/// Managed by [`play_animations`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PlayerAnimationState {
    None,
    Idle,
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_animations(
    mut query: Query<(
        &mut TnuaAnimatingState<PlayerAnimationState>,
        &AnimationPlayers,
    )>,
    mut q_animation: Query<(
        &PlayerAnimations,
        &mut AnimationPlayer,
        &mut AnimationTransitions,
    )>,
    crosshair_state: Single<&CrosshairState>,
) {
    for (mut animating_state, anim_players) in &mut query {
        let mut iter = q_animation.iter_many_mut(anim_players.iter());
        while let Some((animations, mut anim_player, mut transitions)) = iter.fetch_next() {
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
                        transitions.play(
                            &mut anim_player,
                            animations.a_pose,
                            Duration::from_millis(400),
                        );
                    }
                    PlayerAnimationState::Idle => {
                        transitions
                            .play(
                                &mut anim_player,
                                animations.idle,
                                Duration::from_millis(150),
                            )
                            .repeat();
                    }
                },
            }
        }
    }
}
