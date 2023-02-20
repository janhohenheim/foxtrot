use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::player_control::camera::IngameCamera;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug,Component, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Skydome;

pub struct SkydomeSpawner;

impl PrimedGameObjectSpawnerImplementor for SkydomeSpawner {
    fn create_mesh(&self, mesh_assets: &mut ResMut<Assets<Mesh>>) -> Option<Handle<Mesh>> {
        Some(mesh_assets.add(Mesh::from(shape::UVSphere {
            radius: 100.0,
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
                MaterialMeshBundle {
                    mesh: spawner.outer_spawner.meshes[&object].clone(),
                    material: spawner.materials.skydome.clone(),
                    ..default()
                },
            ))
            .id())
    }
}
