use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::movement::general_movement::{CharacterAnimations, KinematicCharacterBundle, Model};
use crate::player_control::player_embodiment::Player;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 1.;
pub const RADIUS: f32 = 0.4;

pub struct PlayerSpawner;

impl PrimedGameObjectSpawnerImplementor for PlayerSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        let gltf = spawner
            .gltf
            .get(&spawner.scenes.character)
            .context("Failed to load scene for player")?;

        Ok(spawner
            .commands
            .spawn((
                PbrBundle {
                    transform,
                    ..default()
                },
                Player,
                Name::new("Player"),
                KinematicCharacterBundle::capsule(HEIGHT, RADIUS),
                CharacterAnimations {
                    idle: spawner.animations.character_idle.clone(),
                    walk: spawner.animations.character_walking.clone(),
                    aerial: spawner.animations.character_running.clone(),
                },
                CollisionGroups::new(
                    GameCollisionGroup::PLAYER.into(),
                    GameCollisionGroup::ALL.into(),
                ),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -HEIGHT, 0.),
                            rotation: Quat::from_rotation_y(TAU / 2.),
                            scale: Vec3::splat(0.01),
                        },
                        ..default()
                    },
                    Model,
                    Name::new("Player Model"),
                ));
            })
            .id())
    }
}
