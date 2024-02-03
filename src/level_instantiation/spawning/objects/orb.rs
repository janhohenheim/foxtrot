use crate::level_instantiation::spawning::objects::util::MeshAssetsExt;
use crate::shader::ShaderMaterials;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Orb;

fn get_or_add_mesh_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: Handle<Mesh> = Handle::weak_from_u128(0x1f40128bac02a9b);
    mesh_assets.get_or_add(MESH_HANDLE, || {
        Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })
    })
}

pub(crate) fn spawn(
    orb: Query<Entity, Added<Orb>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<ShaderMaterials>,
    children: Query<&Children>,
) {
    for entity in orb.iter() {
        let mesh_handle = get_or_add_mesh_handle(&mut meshes);
        children.iter_descendants(entity).for_each(|child| {
            commands.entity(child).despawn_recursive();
        });
        commands
            .entity(entity)
            .insert((
                MaterialMeshBundle {
                    mesh: mesh_handle,
                    material: materials.glowy.clone(),
                    ..default()
                },
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .with_children(|parent| {
                parent.spawn((PointLightBundle {
                    point_light: PointLight {
                        intensity: 10_000.,
                        radius: 1.,
                        color: Color::rgb(0.5, 0.1, 0.),
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },));
            });
    }
}
