use crate::{level_instantiation::spawning::objects::*, GameState};
use bevy::prelude::*;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_registry_export::ExportRegistryPlugin;
use bevy_xpbd_3d::PhysicsSet;
use serde::{Deserialize, Serialize};

pub(crate) mod objects;

/// Handles the runtime spawning of objects loaded from GLTFs.
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
pub(crate) fn spawning_plugin(app: &mut App) {
    app.add_plugins((
        BlueprintsPlugin {
            library_folder: "scenes/library".into(),
            format: GltfFormat::GLB,
            aabbs: true,
            legacy_mode: false,
            ..Default::default()
        },
        ExportRegistryPlugin {
            save_path: "assets/scenes/registry.json".into(),
            ..Default::default()
        },
    ))
    .register_type::<camera::IngameCameraMarker>()
    .register_type::<orb::Orb>()
    .register_type::<sunlight::Sun>()
    .register_type::<Hidden>()
    .register_type::<ground::Grass>()
    .add_systems(
        Update,
        (
            ground::spawn,
            camera::spawn,
            orb::spawn,
            player::spawn,
            npc::spawn,
            sunlight::spawn,
            hide.after(PhysicsSet::Sync),
        )
            .run_if(in_state(GameState::Playing)),
    );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Hidden;

fn hide(hidden: Query<Entity, Added<Hidden>>, mut commands: Commands) {
    for entity in hidden.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
