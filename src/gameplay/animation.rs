//! Animation system boilerplate.

use std::iter;

use bevy::{prelude::*, scene::SceneInstanceReady};
pub(super) fn plugin(app: &mut App) {
    app.register_type::<AnimationPlayerLink>();
    app.add_observer(link_animation_player);
}

/// Entities with this component will receive an [`AnimationPlayerLink`] component so that they can easily find the animation player of their model.
#[derive(Component)]
pub(crate) struct AnimationPlayerAncestor;

/// Simple link to the animation player of a model that is buried deep in the hierarchy.
#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub(crate) struct AnimationPlayerLink(pub(crate) Entity);

/// Bevy likes to hide the [`AnimationPlayer`] component deep in the hierarchy of a model.
/// This system ensures that we can find the animation player easily by inserting an [`AnimationPlayerLink`] component
/// into the same entity that contains the [`AnimationPlayerAncestor`] component.
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
