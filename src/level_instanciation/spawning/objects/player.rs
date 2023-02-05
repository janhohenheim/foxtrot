use crate::level_instanciation::spawning::PrimedGameObjectSpawner;
use crate::movement::general_movement::{CharacterAnimations, KinematicCharacterBundle, Model};
use crate::player_control::camera::PlayerCamera;
use crate::player_control::player_embodiment::{Player, PlayerSensor};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 1.;
pub const RADIUS: f32 = 0.4;
pub const SCALE: f32 = 0.5;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_player(&'a mut self) {
        let gltf = self
            .gltf
            .get(&self.scenes.character)
            .unwrap_or_else(|| panic!("Failed to load scene for player"));

        self.commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_scale(Vec3::splat(SCALE)),
                    ..default()
                },
                Player,
                Name::new("Player"),
                KinematicCharacterBundle::capsule(HEIGHT, RADIUS),
                CharacterAnimations {
                    idle: self.animations.character_idle.clone(),
                    walk: self.animations.character_walking.clone(),
                    aerial: self.animations.character_running.clone(),
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
                    PlayerCamera,
                    Camera3dBundle {
                        transform: Transform::from_xyz(10., 2., 0.),
                        ..default()
                    },
                    Name::new("Player Camera"),
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
