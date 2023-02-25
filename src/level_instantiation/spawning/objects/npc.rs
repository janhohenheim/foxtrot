use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::{DialogId, DialogTarget};
use anyhow::Result;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 0.4;
pub const RADIUS: f32 = 0.4;

pub struct NpcSpawner;

impl PrimedGameObjectSpawnerImplementor for NpcSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                PbrBundle {
                    transform,
                    ..default()
                },
                Name::new("NPC"),
                CharacterControllerBundle::capsule(HEIGHT, RADIUS),
                Follower,
                CharacterAnimations {
                    idle: spawner.animations.character_idle.clone(),
                    walk: spawner.animations.character_walking.clone(),
                    aerial: spawner.animations.character_running.clone(),
                },
                DialogTarget {
                    dialog_id: DialogId::new("follower"),
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("NPC Dialog Collider"),
                    Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                    CollisionGroups::new(
                        GameCollisionGroup::OTHER.into(),
                        GameCollisionGroup::PLAYER.into(),
                    ),
                ));
                parent.spawn((
                    SceneBundle {
                        scene: spawner.scene_handles.character.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                            scale: Vec3::splat(0.012),
                            rotation: Quat::from_rotation_y(TAU / 2.),
                        },
                        ..default()
                    },
                    Model,
                    Name::new("NPC Model"),
                ));
            })
            .id())
    }
}
