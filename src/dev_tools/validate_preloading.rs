//! Validates that all assets are preloaded before the game starts.

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(validate_mesh);
    app.add_observer(validate_material);
    app.add_observer(validate_scene);
    app.add_observer(validate_audio);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn validate_mesh(
    trigger: Trigger<OnAdd, Mesh3d>,
    q_mesh: Query<&Mesh3d>,
    assets: Res<AssetServer>,
) {
    let handle = &q_mesh.get(trigger.target()).unwrap().0;
    validate_asset(handle, &assets, "Mesh");
}

#[cfg_attr(feature = "hot_patch", hot)]
fn validate_material(
    trigger: Trigger<OnAdd, MeshMaterial3d<StandardMaterial>>,
    q_material: Query<&MeshMaterial3d<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let handle = &q_material.get(trigger.target()).unwrap().0;
    validate_asset(handle, &assets, "Material");
}

#[cfg_attr(feature = "hot_patch", hot)]
fn validate_scene(
    trigger: Trigger<OnAdd, SceneRoot>,
    q_scene: Query<&SceneRoot>,
    assets: Res<AssetServer>,
) {
    let handle = &q_scene.get(trigger.target()).unwrap().0;
    validate_asset(handle, &assets, "Scene");
}

#[cfg_attr(feature = "hot_patch", hot)]
fn validate_asset<T: Asset>(handle: &Handle<T>, assets: &AssetServer, type_name: &str) {
    let Some(path) = handle.path() else {
        return;
    };
    if !assets.is_loaded_with_dependencies(handle) {
        warn!("{type_name} at path \"{path}\" was not preloaded and will load during gameplay.",);
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn validate_audio(
    trigger: Trigger<OnAdd, AudioPlayer>,
    q_audio: Query<&AudioPlayer>,
    assets: Res<AssetServer>,
) {
    let handle = &q_audio.get(trigger.target()).unwrap().0;
    validate_asset(handle, &assets, "Audio");
}
