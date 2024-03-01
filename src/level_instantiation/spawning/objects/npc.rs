use crate::{
    level_instantiation::spawning::objects::{player, CollisionLayer},
    movement::{
        character_controller::{CharacterAnimations, CharacterControllerBundle},
        navigation::Follower,
    },
    world_interaction::dialog::YarnNode,
};
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub(crate) fn spawn(
    follower: Query<(Entity, &Transform), Added<Follower>>,
    mut commands: Commands,
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
                // Use the same names as in Blender
                CharacterAnimations {
                    idle: "Idle".into(),
                    walk: "Walk".into(),
                    aerial: "Run".into(),
                },
                YarnNode("Follower".to_string()),
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
