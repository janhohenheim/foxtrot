//! Utility functions for creating regular props that don't have any special properties.
//! A *dynamic* prop in the context of this file is a prop that is influenced by physics,
//! while a *static* prop is unmovable terrain.

use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel as _;
use crate::third_party::{avian3d::CollisionLayer, bevy_trenchbroom::fix_gltf_rotation};
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_tnua::TnuaNotPlatform;
use bevy_trenchbroom::{class::QuakeClass, prelude::*};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_static_prop_with_convex_hull<T: QuakeClass>(
    mut world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    if world.is_scene_world() {
        return;
    }

    let bundle = static_bundle::<T>(&world, ColliderConstructor::ConvexHullFromMesh);
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert(bundle);
}

pub(crate) fn setup_static_prop_with_convex_decomposition<T: QuakeClass>(
    mut world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    if world.is_scene_world() {
        return;
    }

    let bundle = static_bundle::<T>(&world, ColliderConstructor::ConvexDecompositionFromMesh);
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert(bundle);
}

pub(crate) fn dynamic_bundle<T: QuakeClass>(
    world: &DeferredWorld,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = world.load_trenchbroom_model::<T>();
    (
        TransformInterpolation,
        ColliderConstructorHierarchy::new(constructor)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
        RigidBody::Dynamic,
        TnuaNotPlatform,
        SceneRoot(model),
    )
}

pub(crate) fn static_bundle<T: QuakeClass>(
    world: &DeferredWorld,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = world.load_trenchbroom_model::<T>();
    (
        ColliderConstructorHierarchy::new(constructor).with_default_layers(CollisionLayers::new(
            CollisionLayer::Default,
            LayerMask::ALL,
        )),
        RigidBody::Static,
        SceneRoot(model),
    )
}
