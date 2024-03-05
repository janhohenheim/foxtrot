use bevy::prelude::*;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_registry_export::ExportRegistryPlugin;

/// Through the [`BlueprintsPlugin`], components can be deserialized from the "extras" field of the GLTF.
/// In Blender, you should use the newest [Bevy Components Addon](https://github.com/kaosat-dev/Blender_bevy_components_workflow/releases?q=bevy_components&expanded=true).
/// See the linked repo for usage instructions.
///
/// The basic workflow is as follows:
/// - Create a scene in Blender
/// - Add marker components to objects via the Bevy Components Addon, e.g. `ColliderMarker`
/// (If using an old version of the plugin, make sure to turn the legacy mode *off*)
/// - Export the scene as a GLTF
/// - Load the GLTF in Bevy
/// - React to objects being spawned with a marker component via a query like `Query<Entity, Added<ColliderMarker>>`
/// - Add the rest of the appropriate components to the entity, e.g. `Collider` and `RigidBody` for a `ColliderMarker`
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        BlueprintsPlugin {
            library_folder: "scenes/library".into(),
            format: GltfFormat::GLB,
            aabbs: true,
            legacy_mode: false,
            ..default()
        },
        ExportRegistryPlugin {
            save_path: "scenes/registry.json".into(),
            ..default()
        },
    ));
}
