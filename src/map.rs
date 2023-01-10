use bevy::prelude::*;

use crate::dialog::{DialogId, DialogTarget};
use crate::loading::{DynamicSceneAssets, SceneAssets};
use crate::spawning::{GameObject, SpawnEvent, SpawnEventSender};
use crate::GameState;
use bevy::gltf::Gltf;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

const GRASS_SIZE: f32 = 10.;

fn load_scene(mut commands: Commands, scenes: Res<DynamicSceneAssets>) {
    commands.spawn(DynamicSceneBundle {
        scene: scenes.demo.clone(),
        ..default()
    });
}
fn setup(mut commands: Commands, mut spawner: EventWriter<SpawnEvent>) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.1,
    });

    let grass_x = 10;
    let grass_z = 10;

    for x in 0..grass_x {
        for z in 0..grass_z {
            SpawnEventSender::new(&mut spawner)
                .object(GameObject::Grass)
                .translation((
                    GRASS_SIZE * (-grass_x / 2 + x) as f32,
                    0.,
                    GRASS_SIZE * (-grass_z / 2 + z) as f32,
                ))
                .parent("Grass Container")
                .send();
        }
    }

    let wall_width = 3.;
    SpawnEventSender::new(&mut spawner)
        .parent("House")
        .object(GameObject::Doorway)
        .send()
        .object(GameObject::Wall)
        .translation((0., 0., wall_width))
        .send()
        .translation((0., 0., -wall_width))
        .send();

    SpawnEventSender::new(&mut spawner)
        .object(GameObject::Sunlight)
        .rotation(Quat::from_rotation_x(-TAU / 8.))
        .send();
    /*

    .spawn_character(Transform {
        translation: Vec3::new(-5., 0.5, 0.),
        scale: Vec3::splat(0.7),
        rotation: Quat::from_rotation_y(TAU / 4.),
    });*/
}

struct PhysicsAssets<'a, 'w, 's> {
    commands: Commands<'w, 's>,
    scenes: Res<'a, SceneAssets>,
    gltf: Res<'a, Assets<Gltf>>,
}

impl<'a, 'w, 's> PhysicsAssets<'a, 'w, 's> {
    fn spawn_character(&mut self, transform: Transform) -> &mut Self {
        let model = self
            .gltf
            .get(&self.scenes.character)
            .expect("Failed to load npc model");

        let height = 1.0;
        let radius = 0.4;
        self.commands
            .spawn((
                PbrBundle {
                    transform,
                    ..default()
                },
                Name::new("NPC"),
                RigidBody::Fixed,
                Collider::capsule_y(height / 2., radius),
            ))
            .with_children(|parent| {
                parent.spawn((
                    DialogTarget {
                        dialog_id: DialogId::new("sample"),
                    },
                    Name::new("NPC Dialog Collider"),
                    Collider::cylinder(height / 2., radius * 5.),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::KINEMATIC_STATIC,
                ));
                parent.spawn((
                    SceneBundle {
                        scene: model.scenes[0].clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -height, 0.),
                            scale: Vec3::splat(0.02),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("NPC Model"),
                ));
            });
        self
    }
}
