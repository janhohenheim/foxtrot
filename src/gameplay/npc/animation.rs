//! NPC animation handling.

use std::time::Duration;

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective, prelude::*};

use crate::{PostPhysicsAppSystems, gameplay::animation::AnimationPlayers, screens::Screen};

use super::assets::NpcAssets;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<NpcAnimations>();
    app.add_systems(
        Update,
        play_animations
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::PlayAnimations),
    );
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct NpcAnimations {
    idle: AnimationNodeIndex,
    walk: AnimationNodeIndex,
    run: AnimationNodeIndex,
}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn setup_npc_animations(
    trigger: Trigger<OnAdd, AnimationPlayers>,
    q_anim_players: Query<&AnimationPlayers>,
    mut commands: Commands,
    assets: Res<NpcAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let anim_players = q_anim_players.get(trigger.target()).unwrap();
    for anim_player in anim_players.iter() {
        let (graph, indices) = AnimationGraph::from_clips([
            assets.run_animation.clone(),
            assets.idle_animation.clone(),
            assets.walk_animation.clone(),
        ]);
        let [run_index, idle_index, walk_index] = indices.as_slice() else {
            unreachable!()
        };
        let graph_handle = graphs.add(graph);

        let animations = NpcAnimations {
            idle: *idle_index,
            walk: *walk_index,
            run: *run_index,
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
pub(crate) enum NpcAnimationState {
    Standing,
    Airborne,
    Walking(f32),
    Running(f32),
}

#[cfg_attr(feature = "hot_patch", hot)]
fn play_animations(
    mut query: Query<(
        &mut TnuaAnimatingState<NpcAnimationState>,
        &TnuaController,
        &AnimationPlayers,
    )>,
    mut q_animation: Query<(
        &NpcAnimations,
        &mut AnimationPlayer,
        &mut AnimationTransitions,
    )>,
) {
    for (mut animating_state, controller, anim_players) in &mut query {
        let mut iter = q_animation.iter_many_mut(anim_players.iter());
        while let Some((animations, mut anim_player, mut transitions)) = iter.fetch_next() {
            match animating_state.update_by_discriminant({
                let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                    continue;
                };
                let speed = basis_state.running_velocity.length();
                if controller.is_airborne().unwrap() {
                    NpcAnimationState::Airborne
                } else if speed > 4.5 {
                    NpcAnimationState::Running(speed)
                } else if speed > 0.01 {
                    NpcAnimationState::Walking(speed)
                } else {
                    NpcAnimationState::Standing
                }
            }) {
                TnuaAnimatingStateDirective::Maintain { state } => {
                    if let NpcAnimationState::Running(speed) | NpcAnimationState::Walking(speed) =
                        state
                    {
                        if let Some((_index, playing_animation)) =
                            anim_player.playing_animations_mut().next()
                        {
                            let anim_speed = (speed / 3.0).max(0.3);
                            playing_animation.set_speed(anim_speed);
                        }
                    }
                }
                TnuaAnimatingStateDirective::Alter {
                    // We don't need the old state here, but it's available for transition
                    // animations.
                    old_state: _,
                    state,
                } => match state {
                    NpcAnimationState::Airborne => {
                        transitions
                            .play(&mut anim_player, animations.run, Duration::from_millis(200))
                            .repeat();
                    }
                    NpcAnimationState::Standing => {
                        transitions
                            .play(
                                &mut anim_player,
                                animations.idle,
                                Duration::from_millis(500),
                            )
                            .repeat();
                    }
                    NpcAnimationState::Walking(_speed) => {
                        transitions
                            .play(
                                &mut anim_player,
                                animations.walk,
                                Duration::from_millis(300),
                            )
                            .repeat();
                    }
                    NpcAnimationState::Running(_speed) => {
                        transitions
                            .play(&mut anim_player, animations.run, Duration::from_millis(400))
                            .repeat();
                    }
                },
            }
        }
    }
}
