use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::prelude::*;

pub struct LevelSpawner;

impl PrimedGameObjectSpawnerImplementor for LevelSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        object: GameObject,
    ) {
        spawner.spawn_gltf(object, &spawner.scenes.level, Transform::default());
    }
}
