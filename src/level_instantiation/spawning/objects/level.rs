use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use anyhow::Result;
use bevy::prelude::*;

pub struct LevelSpawner;

impl PrimedGameObjectSpawnerImplementor for LevelSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                SceneBundle {
                    scene: spawner.scene_handles.level.clone(),
                    transform,
                    ..default()
                },
                Name::new(format!("{object:?}")),
                Imported,
            ))
            .id())
    }
}

#[derive(Component)]
pub struct Imported;
