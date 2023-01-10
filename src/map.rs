use bevy::prelude::*;

use crate::dialog::{DialogId, DialogTarget};
use crate::game_objects::GameObjects;
use crate::loading::{DynamicSceneAssets, SceneAssets};
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
fn setup(
    mut commands: Commands,
    scenes: Res<SceneAssets>,
    gltf: Res<Assets<Gltf>>,
    game_objects: Res<GameObjects>,
    asset_server: Res<AssetServer>,
) {
    let grass_x = 10;
    let grass_z = 10;
    let game_objects = game_objects.retrieve_with(&asset_server, &mut commands);
    /*
    for x in 0..grass_x {
        for z in 0..grass_z {
            commands.spawn(game_objects.grass(Transform::from_xyz(
                GRASS_SIZE * (-grass_x / 2 + x) as f32,
                0.,
                GRASS_SIZE * (-grass_z / 2 + z) as f32,
            )));
        }
    }
    let mut physics_assets = PhysicsAssets {
        commands,
        scenes,
        gltf,
    };
    let wall_width = 1.;
    let scale = 3.;
    physics_assets
        .spawn_doorway(Transform::from_scale(Vec3::splat(scale)))
        .spawn_wall(Transform {
            translation: Vec3::new(0., 0., wall_width * scale),
            scale: Vec3::splat(scale),
            ..default()
        })
        .spawn_wall(Transform {
            translation: Vec3::new(0., 0., -wall_width * scale),
            scale: Vec3::splat(scale),
            ..default()
        })
        .spawn_light(Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-TAU / 8.),
            ..default()
        })
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
    fn spawn_doorway(&mut self, transform: Transform) -> &mut Self {
        if let Some(gltf) = self.gltf.get(&self.scenes.wall_wood_doorway_round) {
            self.commands
                .spawn((
                    SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform,
                        ..default()
                    },
                    Name::new("Wall Wood Doorway Round"),
                ))
                .with_children(|parent| {
                    let offset = 0.002;
                    parent.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(
                            -0.45,
                            0.5,
                            0.5 / 3. * 2. + 5. * offset,
                        )),
                        Collider::cuboid(0.04, 0.5, 0.5 / 3. + offset),
                    ));
                    parent.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(
                            -0.45,
                            0.5,
                            -0.5 / 3. * 2. - 5. * offset,
                        )),
                        Collider::cuboid(0.04, 0.5, 0.5 / 3. + offset),
                    ));
                });
        }
        self
    }

    fn spawn_wall(&mut self, transform: Transform) -> &mut Self {
        if let Some(gltf) = self.gltf.get(&self.scenes.wall_wood) {
            self.commands
                .spawn((
                    SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform,
                        ..default()
                    },
                    Name::new("Wall Wood"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(-0.45, 0.5, 0.)),
                        Collider::cuboid(0.04, 0.5, 0.5),
                    ));
                });
        }
        self
    }

    fn spawn_light(&mut self, transform: Transform) -> &mut Self {
        self.commands.insert_resource(AmbientLight {
            color: Color::ORANGE_RED,
            brightness: 0.1,
        });

        // directional 'sun' light
        const HALF_SIZE: f32 = 10.0;
        self.commands.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    // Configure the projection to better fit the scene
                    shadow_projection: OrthographicProjection {
                        left: -HALF_SIZE,
                        right: HALF_SIZE,
                        bottom: -HALF_SIZE,
                        top: HALF_SIZE,
                        near: -10.0 * HALF_SIZE,
                        far: 10.0 * HALF_SIZE,
                        ..default()
                    },
                    shadows_enabled: true,
                    ..default()
                },
                transform,
                ..default()
            },
            Name::new("Light"),
        ));
        self
    }

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
