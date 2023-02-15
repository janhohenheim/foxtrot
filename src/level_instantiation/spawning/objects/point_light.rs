use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use anyhow::Result;
use bevy::prelude::*;

pub struct PointLightSpawner;

impl PrimedGameObjectSpawnerImplementor for PointLightSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1.0,
                        range: 1.0,
                        radius: 1.0,
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
