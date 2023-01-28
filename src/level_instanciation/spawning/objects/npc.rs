use crate::level_instanciation::spawning::post_spawn_modification::CustomCollider;
use crate::level_instanciation::spawning::{GameObject, PrimedGameObjectSpawner};
use crate::movement::general_movement::{
    CharacterAnimations, CharacterVelocity, Grounded, Jump, Model,
};
use crate::movement::navigation::Follower;
use crate::world_interaction::dialog::{DialogId, DialogTarget};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const PATH: &str = "scenes/Fox.glb";

pub fn load_scene(asset_server: &Res<AssetServer>) -> Handle<Gltf> {
    asset_server.load(PATH)
}

pub const HEIGHT: f32 = 1.;
pub const RADIUS: f32 = 0.4;
pub const SCALE: f32 = 0.6;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_npc(&'a mut self) {
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
                Name::new("NPC"),
                RigidBody::KinematicVelocityBased,
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
                CharacterVelocity::default(),
                Grounded::default(),
                Jump::default(),
                Follower,
                CharacterAnimations {
                    idle: self.animations.character_idle.clone(),
                    walk: self.animations.character_walking.clone(),
                    aerial: self.animations.character_running.clone(),
                },
                Collider::capsule_y(HEIGHT / 2., RADIUS),
            ))
            .with_children(|parent| {
                parent.spawn((
                    DialogTarget {
                        dialog_id: DialogId::new("sample"),
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
