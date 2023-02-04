use crate::level_instanciation::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::prelude::*;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_level(&'a mut self) {
        self.spawn_gltf(GameObject::Level, &self.scenes.level, Transform::default());
    }
}
