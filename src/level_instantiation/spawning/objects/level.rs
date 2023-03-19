use crate::file_system_interaction::asset_loading::SceneAssets;
use crate::level_instantiation::spawning::GameObject;
use bevy::prelude::*;

pub(crate) fn spawn(world: &mut World, transform: Transform) {
    let scene_handles = world.get_resource::<SceneAssets>().unwrap().clone();
    world.spawn((
        SceneBundle {
            scene: scene_handles.level,
            transform,
            ..default()
        },
        Name::new("Level"),
        Imported,
        GameObject::Level,
    ));
}

#[derive(Component)]
pub struct Imported;
