use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::prelude::*;

pub struct LevelSpawner;

impl PrimedGameObjectSpawnerImplementor for LevelSpawner {
    fn spawn(&self, spawner: &mut PrimedGameObjectSpawner, object: GameObject) {
        spawner.spawn_gltf(object, &spawner.scenes.level, Transform::default());
    }
}
