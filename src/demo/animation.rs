//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use std::iter;

use bevy::{prelude::*, scene::SceneInstanceReady};
pub(super) fn plugin(app: &mut App) {
    app.register_type::<AnimationPlayerLink>();
    app.add_observer(link_animation_player);
}

#[derive(Component)]
pub(crate) struct AnimationPlayerAncestor;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct AnimationPlayerLink(pub(crate) Entity);

fn link_animation_player(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    q_parent: Query<&Parent>,
    q_children: Query<&Children>,
    q_animation_player: Query<Entity, With<AnimationPlayer>>,
    q_ancestor: Query<Entity, With<AnimationPlayerAncestor>>,
) {
    let scene_root = trigger.entity();
    let animation_player = q_children
        .iter_descendants(scene_root)
        .find(|child| q_animation_player.get(*child).is_ok());
    let Some(animation_player) = animation_player else {
        return;
    };

    let animation_ancestor = iter::once(animation_player)
        .chain(q_parent.iter_ancestors(animation_player))
        .find(|entity| q_ancestor.get(*entity).is_ok());
    let Some(animation_ancestor) = animation_ancestor else {
        return;
    };

    commands
        .entity(animation_ancestor)
        .insert(AnimationPlayerLink(animation_player));
}
