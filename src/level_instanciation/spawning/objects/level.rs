use crate::level_instanciation::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub const PATH: &str = "scenes/old_town.glb";

pub fn load_scene(asset_server: &Res<AssetServer>) -> Handle<Gltf> {
    asset_server.load(PATH)
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_level(&'a mut self) {
        self.spawn_gltf(GameObject::Level, Transform::default());
    }
}
