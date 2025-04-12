use bevy::prelude::*;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective, prelude::*};

use crate::{
    demo::animation::{AnimationPlayerLink, LoadModelAnimation as _, ModelAnimation},
    screens::Screen,
    third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

use super::Npc;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<NpcAnimations>();
    app.add_observer(setup_npc_animations);
    app.add_systems(Update, play_animations.run_if(in_state(Screen::Gameplay)));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct NpcAnimations {
    idle: AnimationNodeIndex,
    walk: AnimationNodeIndex,
    run: AnimationNodeIndex,
}

fn setup_npc_animations(
    trigger: Trigger<OnAdd, AnimationPlayerLink>,
    q_npc: Query<&AnimationPlayerLink, With<Npc>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let anim_player_link = trigger.entity();
    let Ok(anim_player_link) = q_npc.get(anim_player_link) else {
        return;
    };
    let anim_player = anim_player_link.0;

    // Create an animation graph containing a single animation. We want the "run"
    // animation from our example asset, which has an index of two.
    let load_animation = |name: &str| -> Handle<AnimationClip> {
        asset_server.load(format!("{}#Animation{}", Npc::model_path(), name))
    };

    let run_handle = load_animation("0");
    let idle_handle = load_animation("1");
    let walk_handle = load_animation("2");

    let (graph, indices) = AnimationGraph::from_clips([run_handle, idle_handle, walk_handle]);
    let [run_index, idle_index, walk_index] = indices.as_slice() else {
        unreachable!()
    };
    // Store the animation graph as an asset.
    let graph_handle = graphs.add(graph);

    let animations = NpcAnimations {
        idle: *idle_index,
        walk: *walk_index,
        run: *run_index,
    };
    commands
        .entity(anim_player)
        .insert((animations, AnimationGraphHandle(graph_handle)));
}

/// Managed by [`play_animations`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NpcAnimationState {
    Standing,
    Airborne,
    Walking(f32),
    Running(f32),
}

fn play_animations(
    mut query: Query<(
        &mut TnuaAnimatingState<NpcAnimationState>,
        &TnuaController,
        &AnimationPlayerLink,
    )>,
    mut q_animation: Query<(&NpcAnimations, &mut AnimationPlayer)>,
) {
    for (mut animating_state, controller, link) in query.iter_mut() {
        let animation_player_entity = link.0;
        let Ok((animations, mut animation_player)) = q_animation.get_mut(animation_player_entity)
        else {
            continue;
        };
        match animating_state.update_by_discriminant({
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                continue;
            };
            let speed = basis_state.running_velocity.length();
            if controller.is_airborne().unwrap() {
                NpcAnimationState::Airborne
            } else if speed > 10.0 {
                NpcAnimationState::Running(speed)
            } else if speed > 0.01 {
                NpcAnimationState::Walking(speed)
            } else {
                NpcAnimationState::Standing
            }
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => {
                if let NpcAnimationState::Running(speed) = state {
                    let anim_speed = (speed / 7.0).max(1.0);
                    //animation_player.set_speed(anim_speed);
                }
            }
            TnuaAnimatingStateDirective::Alter {
                // We don't need the old state here, but it's available for transition
                // animations.
                old_state: _,
                state,
            } => match state {
                NpcAnimationState::Airborne | NpcAnimationState::Running(..) => {
                    info!("playing run animation");
                    animation_player.play(animations.run).repeat();
                }
                NpcAnimationState::Standing => {
                    info!("playing idle animation");
                    animation_player.play(animations.idle).repeat();
                }
                NpcAnimationState::Walking(_speed) => {
                    info!("playing walk animation");
                    animation_player.play(animations.walk).repeat();
                }
            },
        }
    }
}
