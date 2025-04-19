use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

use crate::{
    props::{Candle, generic::dynamic_bundle},
    third_party::avian3d::CollisionLayer,
};
pub(crate) use burning_logs::*;

use super::Chair;

mod burning_logs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(burning_logs::plugin);
}

pub(crate) fn setup_candle(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle = dynamic_bundle::<Candle>(&world);
    world.commands().entity(entity).insert(bundle);
}

pub(crate) fn setup_chair(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle = dynamic_bundle::<Chair>(&world);
    world.commands().entity(entity).insert(bundle).insert(
        // Use a convex decomposition, as otherwise the flat seat part of the chair would not exist.
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // Make the chair way more dense than the default, as it feels janky to be able to push it around easily.
            .with_default_density(10_000.0),
    );
}
