use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Grass;

pub(crate) fn spawn(
    sun: Query<&Children, Added<Grass>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for children in sun.iter() {
        for child in children.iter() {
            if let Ok(material_handle) = material_handles.get(*child) {
                let material = materials.get_mut(material_handle).unwrap();
                material.reflectance = 0.05;
            }
        }
    }
}
