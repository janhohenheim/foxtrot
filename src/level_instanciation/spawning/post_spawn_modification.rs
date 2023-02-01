use crate::shader::{Materials, RepeatedMaterial, Repeats};
use crate::util::trait_extension::MeshExt;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CustomCollider;

#[allow(clippy::type_complexity)]
pub fn read_colliders(
    mut commands: Commands,
    added_name: Query<(&Name, &Children), (Added<Name>, Without<CustomCollider>)>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    for (name, children) in &added_name {
        if name.to_lowercase().contains("[collider]") {
            for (collider_entity, collider_mesh) in
                Mesh::search_in_children(children, &meshes, &mesh_handles)
            {
                let rapier_collider =
                    Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                        .unwrap();

                commands.entity(collider_entity).insert(rapier_collider);
            }
        }
    }
}

pub fn set_texture_to_repeat(
    mut commands: Commands,
    added_name: Query<(&Name, &Children), Added<Name>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Materials>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut repeated_materials: ResMut<Assets<RepeatedMaterial>>,
) {
    for (name, children) in &added_name {
        if name.to_lowercase().contains("[repeat]") {
            for child in children.iter() {
                if let Ok(standard_material_handle) = material_handles.get(*child) {
                    let standard_material =
                        standard_materials.get(standard_material_handle).unwrap();
                    let texture = standard_material.base_color_texture.as_ref().unwrap();
                    let repeated_material = materials
                        .repeated
                        .entry(texture.clone())
                        .or_insert_with(|| {
                            repeated_materials.add(RepeatedMaterial {
                                texture: Some(texture.clone()),
                                repeats: Repeats {
                                    horizontal: 100.0,
                                    vertical: 100.0,
                                },
                            })
                        });

                    commands
                        .entity(*child)
                        .remove::<Handle<StandardMaterial>>()
                        .insert(repeated_material.clone());
                }
            }
        }
    }
}
