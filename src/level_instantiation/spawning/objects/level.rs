use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::prelude::*;

pub struct LevelSpawner;

impl PrimedGameObjectSpawnerImplementor for LevelSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        object: GameObject,
        transform: Transform,
    ) -> Entity {
        spawner
            .spawn_gltf(object, &spawner.scenes.level, transform)
            .id()
    }
}
