use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vect::new(0.0, -9.81 * 30., 0.0),
                ..default()
            });
        #[cfg(debug_assertions)]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
