use crate::file_system_interaction::asset_loading::AnimationAssets;
use crate::level_instantiation::spawning::objects::player;
use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::DialogTarget;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub(crate) fn spawn(
    follower: Query<(Entity, &Transform), Added<Follower>>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
) {
    for (entity, transform) in follower.iter() {
        commands
            .entity(entity)
            .insert((
                CharacterControllerBundle::capsule(
                    player::HEIGHT,
                    player::RADIUS,
                    transform.scale.y,
                ),
                Follower,
                CharacterAnimations {
                    idle: animations.character_idle.clone(),
                    walk: animations.character_walking.clone(),
                    aerial: animations.character_running.clone(),
                },
                DialogTarget {
                    speaker: "The Follower".to_string(),
                    node: "Follower".to_string(),
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("NPC Dialog Collider"),
                    Collider::cylinder(player::HEIGHT / 2., player::RADIUS * 5.),
                    CollisionLayers::new([CollisionLayer::Sensor], [CollisionLayer::Player]),
                    Sensor,
                ));
            });
    }
}
