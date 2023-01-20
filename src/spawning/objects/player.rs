use super::npc::PATH;
use crate::camera::PlayerCamera;
use crate::player::{CharacterVelocity, Grounded, Jump, Model, Player, PlayerSensor};
use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::core_pipeline::bloom::BloomSettings;
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
            .get(&self.handles.scenes[&GameObject::Npc])
            .unwrap_or_else(|| panic!("Failed to load scene from {PATH}"));

        self.commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_scale(Vec3::splat(SCALE)),
                    ..default()
                },
                RigidBody::KinematicVelocityBased,
                Collider::capsule_y(HEIGHT / 2., RADIUS),
                KinematicCharacterController {
                    // Don’t allow climbing slopes larger than n degrees.
                    max_slope_climb_angle: 45.0_f32.to_radians() as Real,
                    // Automatically slide down on slopes smaller than n degrees.
                    min_slope_slide_angle: 30.0_f32.to_radians() as Real,
                    // The character offset is set to n multiplied by the collider’s height.
                    offset: CharacterLength::Relative(2e-2),
                    // Snap to the ground if the vertical distance to the ground is smaller than n.
                    snap_to_ground: Some(CharacterLength::Absolute(1e-3)),
                    filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                },
                Player,
                Name::new("Player"),
                Grounded::default(),
                CharacterVelocity::default(),
                Jump::default(),
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
