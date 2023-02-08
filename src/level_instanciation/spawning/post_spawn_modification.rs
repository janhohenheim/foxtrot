use crate::level_instanciation::spawning::spawn::Despawn;
use crate::shader::{Materials, RepeatedMaterial, Repeats};
use crate::util::trait_extension::MeshExt;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CustomCollider;

#[allow(clippy::type_complexity)]
pub fn read_colliders(
    mut commands: Commands,
    added_name: Query<(Entity, &Name), (Added<Name>, Without<CustomCollider>)>,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    for (entity, name) in &added_name {
        if name.to_lowercase().contains("[collider]") {
            for (collider_entity, collider_mesh) in
                Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
            {
                let rapier_collider =
                    Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                        .unwrap();

                commands.entity(collider_entity).insert(rapier_collider);
            }
        }
    }
}

pub fn set_hidden(mut added_name: Query<(&Name, &mut Visibility), Added<Name>>) {
    for (name, mut visibility) in added_name.iter_mut() {
        if name.to_lowercase().contains("[hidden]") {
            visibility.is_visible = false;
        }
    }
}

pub fn despawn_removed(
    mut commands: Commands,
    mut added_name: Query<(Entity, &Name), Added<Name>>,
) {
    for (entity, name) in added_name.iter_mut() {
        if name.to_lowercase().contains("[remove]") {
            commands.entity(entity).insert(Despawn { recursive: true });
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
    let re = Regex::new(r"\[repeat:(\d+),(\d+)\]").unwrap();
    for (name, children) in &added_name {
        if let Some(captures) = re.captures(&name.to_lowercase()) {
            let repeats = Repeats {
                horizontal: captures[1].parse().unwrap(),
                vertical: captures[2].parse().unwrap(),
            };
            for child in children.iter() {
                if let Ok(standard_material_handle) = material_handles.get(*child) {
                    let standard_material =
                        standard_materials.get(standard_material_handle).unwrap();
                    let texture = standard_material.base_color_texture.as_ref().unwrap();
                    let key = (texture.id(), repeats);

                    let repeated_material = materials.repeated.entry(key).or_insert_with(|| {
                        repeated_materials.add(RepeatedMaterial {
                            texture: Some(texture.clone()),
                            repeats,
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
