use bevy::{asset::AssetPath, prelude::*};
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
    idle: ModelAnimation,
    walk: ModelAnimation,
    run: ModelAnimation,
}

fn setup_npc_animations(
    trigger: Trigger<OnAdd, Npc>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let npc = trigger.entity();

    // Create an animation graph containing a single animation. We want the "run"
    // animation from our example asset, which has an index of two.
    let mut load_animation = |name: &str| {
        asset_server.load_model_animation(
            format!("{}#Animation{}", Npc::model_path(), name),
            &mut graphs,
        )
    };
    let animations = NpcAnimations {
        idle: load_animation("1"),
        walk: load_animation("2"),
        run: load_animation("0"),
    };
    commands.entity(npc).insert(animations);
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
    mut commands: Commands,
    mut query: Query<(
        &mut TnuaAnimatingState<NpcAnimationState>,
        &TnuaController,
        &AnimationPlayerLink,
        &NpcAnimations,
    )>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (mut animating_state, controller, link, animations) in query.iter_mut() {
        let animation_player_entity = link.0;
        let mut animation_player = animation_players.get_mut(animation_player_entity).unwrap();
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
                    animation_player.play(animations.run.index).repeat();
                    commands
                        .entity(animation_player_entity)
                        .insert(AnimationGraphHandle(animations.run.graph_handle.clone()));
                }
                NpcAnimationState::Standing => {
                    info!("playing idle animation");
                    animation_player.play(animations.idle.index).repeat();
                    commands
                        .entity(animation_player_entity)
                        .insert(AnimationGraphHandle(animations.idle.graph_handle.clone()));
                }
                NpcAnimationState::Walking(_speed) => {
                    info!("playing walk animation");
                    animation_player.play(animations.walk.index).repeat();
                    commands
                        .entity(animation_player_entity)
                        .insert(AnimationGraphHandle(animations.walk.graph_handle.clone()));
                }
            },
        }
    }
}
