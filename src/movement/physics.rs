use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

/// Sets up and configures the XPBD physics.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        // Using the default fixed timestep causes issues on faster (165 Hz) machines.
        .insert_resource(Time::new_with(Physics::variable(1.0 / 60.)));
}

#[derive(PhysicsLayer)]
pub(crate) enum CollisionLayer {
    Player,
    Character,
    Terrain,
    CameraObstacle,
    Sensor,
}
