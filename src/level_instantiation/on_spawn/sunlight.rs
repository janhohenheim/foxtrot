use crate::GameState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Sun;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Sun>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}
fn spawn(sun: Query<&Children, Added<Sun>>, mut directional_lights: Query<&mut DirectionalLight>) {
    for children in sun.iter() {
        for child in children.iter() {
            if let Ok(mut light) = directional_lights.get_mut(*child) {
                light.shadows_enabled = true;
            }
        }
    }
}
