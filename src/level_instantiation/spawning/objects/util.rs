use crate::level_instantiation::spawning::{GameObject, PrimedGameObjectSpawner};
use anyhow::{Context, Result};
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;

impl<'w, 's, 'a> PrimedGameObjectSpawner<'w, 's, 'a> {
    pub fn spawn_gltf(
        &'a mut self,
        object: GameObject,
        handle: &Handle<Gltf>,
        transform: Transform,
    ) -> Result<EntityCommands<'w, 's, 'a>> {
        let gltf = self
            .gltf
            .get(handle)
            .context("Failed to load scene for {object:?}")?;
        Ok(self.commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform,
                ..default()
            },
            Name::new(format!("{object:?}")),
        )))
    }
}
