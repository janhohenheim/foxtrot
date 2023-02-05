use crate::level_instanciation::spawning::post_spawn_modification::CustomCollider;
use crate::level_instanciation::spawning::PrimedGameObjectSpawner;
use crate::movement::general_movement::{CharacterAnimations, KinematicCharacterBundle, Model};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::{DialogId, DialogTarget};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 1.;
pub const RADIUS: f32 = 0.4;
pub const SCALE: f32 = 0.6;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_npc(&'a mut self) {
        let gltf = self
            .gltf
            .get(&self.scenes.character)
            .unwrap_or_else(|| panic!("Failed to load scene for NPC"));

        self.commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_scale(Vec3::splat(SCALE)),
                    ..default()
                },
                Name::new("NPC"),
                KinematicCharacterBundle::capsule(HEIGHT, RADIUS),
                Follower,
                CharacterAnimations {
                    idle: self.animations.character_idle.clone(),
                    walk: self.animations.character_walking.clone(),
                    aerial: self.animations.character_running.clone(),
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    DialogTarget {
                        dialog_id: DialogId::new("follower"),
                    },
                    Name::new("NPC Dialog Collider"),
                    Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::KINEMATIC_STATIC,
                    CustomCollider,
                ));
                parent.spawn((
                    SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -HEIGHT, 0.),
                            scale: Vec3::splat(0.02),
                            rotation: Quat::from_rotation_y(TAU / 2.),
                        },
                        ..default()
                    },
                    Model,
                    Name::new("NPC Model"),
                ));
            });
    }
}
