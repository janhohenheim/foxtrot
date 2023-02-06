use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::movement::general_movement::{CharacterAnimations, KinematicCharacterBundle, Model};
use crate::player_control::player_embodiment::{Player, PlayerSensor};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 1.;
pub const RADIUS: f32 = 0.4;
pub const SCALE: f32 = 0.5;

pub struct PlayerSpawner;

impl PrimedGameObjectSpawnerImplementor for PlayerSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
        let gltf = spawner
            .gltf
            .get(&spawner.scenes.character)
            .unwrap_or_else(|| panic!("Failed to load scene for player"));

        spawner
            .commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_scale(Vec3::splat(SCALE)),
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
            ))
            .with_children(|parent| {
                parent.spawn((
                    Collider::capsule_y(HEIGHT / 2., RADIUS),
                    Sensor,
                    PlayerSensor,
                    ActiveCollisionTypes::all(),
                    Name::new("Player Sensor"),
                ));
                parent.spawn((
                    SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -HEIGHT, 0.),
                            rotation: Quat::from_rotation_y(TAU / 2.),
                            scale: Vec3::splat(0.02),
                        },
                        ..default()
                    },
                    Model,
                    Name::new("Player Model"),
                ));
            });
    }
}
