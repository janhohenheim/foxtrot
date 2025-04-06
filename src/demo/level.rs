//! Spawn the main level.

use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::{prelude::PointClass, util::TrenchBroomGltfRotationFix};

use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(world: &mut World) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let asset_server = world.resource::<AssetServer>();
    world.spawn(SceneRoot(
        //  Run ./scripts/compile_maps.sh and change .map to .bsp when you're done prototyping and want some extra performance
        asset_server.load("maps/foxtrot/foxtrot.map#Scene"),
    ));
}

#[derive(PointClass, Component, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/suzanne/Suzanne.gltf")]
#[component(on_add = Self::on_add)]
pub(crate) struct Suzanne;

impl Suzanne {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };

        let suzanne = asset_server.load_trenchbroom_model::<Self>();

        world.commands().entity(entity).insert((
            SceneRoot(suzanne),
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
            TrenchBroomGltfRotationFix,
        ));
    }
}
