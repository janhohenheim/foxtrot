use bevy::prelude::*;

use crate::loading::MaterialAssets;
use crate::GameState;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vect::new(0.0, -9.81 * 30., 0.0),
                ..default()
            })
            .add_startup_system(setup_graphics)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_physics));

        #[cfg(debug_assertions)]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn(Camera3dBundle::default())
        .insert(Name::new("Camera"));
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialAssets>,
) {
    commands.spawn((
        Collider::cuboid(500.0, 50.0, 5_000.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(500., 50., 5_000.))),
            material: materials.grass.clone(),
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Collider::cuboid(500.0, 50.0, 5_000.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(500., 50., 5_000.))),
            material: materials.grass.clone(),
            transform: Transform {
                translation: Vec3::new(400.0, 0.0, 0.0),
                rotation: Quat::from_rotation_z(TAU / 12.),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Collider::cuboid(500.0, 50.0, 5_000.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(500., 50., 5_000.))),
            material: materials.grass.clone(),
            transform: Transform {
                translation: Vec3::new(-400.0, 0.0, 0.0),
                rotation: Quat::from_rotation_z(-TAU / 5.),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(200.0, 10.0, 0.0)),
    ));

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
