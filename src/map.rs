use bevy::prelude::*;

use crate::loading::{MaterialAssets, SceneAssets};
use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialAssets>,
    scenes: Res<SceneAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&scenes.wall_wood_doorway_round) {
        info!("Spawning door");
        // spawn the first scene in the file
        commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform {
                    scale: Vec3::splat(1_000.0),
                    ..default()
                },
                ..default()
            },
            //Collider::cuboid(50.0, 50.0, 50.0),
        ));
    }

    spawn_grass(
        &mut commands,
        &mut meshes,
        &materials,
        Transform::from_xyz(0., -150., 0.),
    );

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
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
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-TAU / 8.),
            ..default()
        },
        ..default()
    });
}

fn spawn_grass<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &Res<MaterialAssets>,
    transform: Transform,
) -> EntityCommands<'w, 's, 'a> {
    let x = 1_024.0 * transform.scale.x;
    let y = 1. * transform.scale.y;
    let z = 1_024.0 * transform.scale.z;
    commands.spawn((
        Collider::cuboid(x / 2., y / 2., z / 2.),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(x, y, z))),
            material: materials.grass.clone(),
            transform,
            ..default()
        },
    ))
}
