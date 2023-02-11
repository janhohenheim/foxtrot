use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct UiCamera;

pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), UiCamera, Name::new("Camera")));
}

pub fn despawn_ui_camera(mut commands: Commands, query: Query<Entity, With<UiCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
