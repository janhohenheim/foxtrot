use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use anyhow::Result;
use bevy::prelude::*;

pub struct SunlightSpawner;

impl PrimedGameObjectSpawnerImplementor for SunlightSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        // directional 'sun' light
        const HALF_SIZE: f32 = 50.0;
        Ok(spawner
            .commands
            .spawn((
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
            ))
            .id())
    }
}
