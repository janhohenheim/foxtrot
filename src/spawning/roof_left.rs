use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub const PATH: &str = "scenes/roofHighLeft.glb";

pub fn load_scene(asset_server: &Res<AssetServer>) -> Handle<Gltf> {
    asset_server.load(PATH)
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_roof_left(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.spawn_gltf(GameObject::RoofLeft, Transform::from_scale(Vec3::splat(3.)))
    }
}
