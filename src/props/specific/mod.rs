use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

use crate::{
    props::generic::dynamic_bundle,
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::fix_gltf_rotation},
};
pub(crate) use burning_logs::*;

use super::Chair;

mod burning_logs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(burning_logs::plugin);
}

pub(crate) fn setup_chair(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle = dynamic_bundle::<Chair>(&world, ColliderConstructor::ConvexDecompositionFromMesh);
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert(bundle)
        .insert(
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh)
                .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                // Make the chair way more dense than the default, as it feels janky to be able to push it around easily.
                .with_default_density(10_000.0),
        );
}
