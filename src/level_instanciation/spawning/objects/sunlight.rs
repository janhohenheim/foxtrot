use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::prelude::*;

pub struct SunlightSpawner;

impl PrimedGameObjectSpawnerImplementor for SunlightSpawner {
    fn spawn(&self, spawner: &mut PrimedGameObjectSpawner, _object: GameObject) {
        // directional 'sun' light
        const HALF_SIZE: f32 = 50.0;
        spawner.commands.spawn((
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
                ..default()
            },
            Name::new("Light"),
        ));
    }
}
