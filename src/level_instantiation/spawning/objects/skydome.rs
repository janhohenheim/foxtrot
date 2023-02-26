use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use anyhow::Result;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, Copy, Serialize, Deserialize, Reflect, FromReflect, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Skydome;

pub struct SkydomeSpawner;

impl PrimedGameObjectSpawnerImplementor for SkydomeSpawner {
    fn create_mesh(&self, mesh_assets: &mut ResMut<Assets<Mesh>>) -> Option<Handle<Mesh>> {
        Some(mesh_assets.add(Mesh::from(shape::UVSphere {
            radius: 150.0,
            ..default()
        })))
    }

    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                Name::new("Skydome"),
                NotShadowCaster,
                NotShadowReceiver,
                Skydome,
                MaterialMeshBundle {
                    mesh: spawner.outer_spawner.meshes[&object].clone(),
                    material: spawner.materials.skydome.clone(),
                    transform,
                    ..default()
                },
            ))
            .id())
    }
}
