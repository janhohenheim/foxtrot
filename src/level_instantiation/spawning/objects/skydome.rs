use crate::level_instantiation::spawning::objects::util::MeshAssetsExt;
use crate::level_instantiation::spawning::GameObject;
use crate::shader::Materials;

use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, Copy, Serialize, Deserialize, Reflect, FromReflect, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Skydome;

fn get_or_add_mesh_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: HandleUntyped =
        HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 0x1f40128bac02a9a);
    mesh_assets.get_or_add(MESH_HANDLE, || {
        Mesh::from(shape::UVSphere {
            radius: 150.0,
            ..default()
        })
    })
}

pub(crate) fn spawn(world: &mut World, transform: Transform) {
    let materials = world.get_resource::<Materials>().unwrap().clone();
    let mesh_handle = {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        get_or_add_mesh_handle(&mut meshes)
    };
    world.spawn((
        Name::new("Skydome"),
        NotShadowCaster,
        NotShadowReceiver,
        Skydome,
        MaterialMeshBundle {
            mesh: mesh_handle,
            material: materials.skydome,
            transform,
            ..default()
        },
        GameObject::Skydome,
    ));
}
