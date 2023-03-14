use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use anyhow::Result;
use bevy::pbr::CascadeShadowConfigBuilder;
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
        Ok(spawner
            .commands
            .spawn((
                DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        shadows_enabled: true,
                        ..default()
                    },
                    cascade_shadow_config: CascadeShadowConfigBuilder {
                        first_cascade_far_bound: 7.0,
                        maximum_distance: 100.0,
                        ..default()
                    }
                    .into(),
                    transform,
                    ..default()
                },
                Name::new("Light"),
            ))
            .id())
    }
}
