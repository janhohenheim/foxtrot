use crate::GameState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Ground;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Ground>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

fn spawn(
    sun: Query<&Children, Added<Ground>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for children in sun.iter() {
        for child in children.iter() {
            if let Ok(material_handle) = material_handles.get(*child) {
                let material = materials.get_mut(material_handle).unwrap();
                // Blender doesn't export this unfortunately, so we'll have to fix the glossy ground manually
                material.reflectance = 0.05;
            }
        }
    }
}
