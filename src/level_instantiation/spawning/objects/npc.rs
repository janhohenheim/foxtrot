use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::DialogTarget;
use bevy::prelude::*;

use bevy_xpbd_3d::prelude::*;
use std::f32::consts::TAU;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.4;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
            Name::new("NPC"),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
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
            GameObject::Npc,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("NPC Dialog Collider"),
                Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                CollisionLayers::new([CollisionLayer::Sensor], [CollisionLayer::Solid]),
                RigidBody::Static,
                Sensor,
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.character.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        scale: Vec3::splat(0.012),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                    },
                    ..default()
                },
                Name::new("NPC Model"),
            ));
        });
}
