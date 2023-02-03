use crate::shader::Repeats;
use crate::util::trait_extension::MeshExt;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_rapier3d::prelude::*;
use regex::Regex;
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
    added_name: Query<(&Name, &Children), Added<Name>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    let re = Regex::new(r"\[repeat:(\d+),(\d+)\]").unwrap();
    for (name, children) in &added_name {
        if let Some(captures) = re.captures(&name.to_lowercase()) {
            let repeats = Repeats {
                horizontal: captures[1].parse().unwrap(),
                vertical: captures[2].parse().unwrap(),
            };
            for mesh_handle in children
                .iter()
                .filter_map(|entity| mesh_handles.get(*entity).ok())
            {
                let collider_mesh = meshes.get_mut(mesh_handle).unwrap();
                let uvs = match collider_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap() {
                    VertexAttributeValues::Float32x2(values) => values,
                    _ => panic!("Unexpected vertex attribute type"),
                };
                for uv in uvs.iter_mut() {
                    uv[0] *= repeats.horizontal as f32;
                    uv[1] *= repeats.vertical as f32;
                }
            }
        }
    }
}
