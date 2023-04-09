use crate::file_system_interaction::asset_loading::SceneAssets;
use crate::level_instantiation::spawning::GameObject;
use bevy::prelude::*;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    scene_handles: Res<SceneAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: scene_handles.level.clone(),
            transform,
            ..default()
        },
        Name::new("Level"),
        Imported,
        GameObject::Level,
    ));
}

#[derive(Component)]
pub(crate) struct Imported;
