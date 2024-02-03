use crate::file_system_interaction::asset_loading::GltfAssets;
use crate::level_instantiation::spawning::objects::player;
use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::movement::character_controller::{CharacterAnimations, CharacterControllerBundle};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::DialogTarget;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub(crate) fn spawn(
    follower: Query<(Entity, &Transform), Added<Follower>>,
    gltf_assets: Res<GltfAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    for (entity, transform) in follower.iter() {
        let level = gltfs.get(gltf_assets.level.clone()).unwrap();
        let animations = &level.named_animations;

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
                    idle: animations["Idle"].clone(),
                    walk: animations["Walk"].clone(),
                    aerial: animations["Run"].clone(),
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
