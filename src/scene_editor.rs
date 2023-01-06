use bevy::prelude::*;

use crate::GameState;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub struct SceneEditorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColliderCreationData {}

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(object_picker));
    }
}

fn object_picker() {}
