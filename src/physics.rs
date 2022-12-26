use bevy::prelude::*;

use crate::GameState;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .insert_resource(RapierConfiguration {
                gravity: Vec2::new(0.0, -9.81 * 30.),
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
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Camera"));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(200.0, 10.0, 0.0)),
    ));
}
