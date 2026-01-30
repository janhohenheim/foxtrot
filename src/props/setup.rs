//! Utility functions for creating regular props that don't have any special properties.
//! A *dynamic* prop in the context of this file is a prop that is influenced by physics,
//! while a *static* prop is unmovable terrain.

use crate::third_party::avian3d::CollisionLayer;
use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel as _;
use avian3d::prelude::*;
use bevy::prelude::*;

use bevy_trenchbroom::class::QuakeClass;

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_static_prop_with_convex_hull<T: QuakeClass>(
    add: On<Add, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = static_bundle::<T>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands.entity(add.entity).insert(bundle);
}

pub(crate) fn setup_nonphysical_prop<T: QuakeClass>(
    add: On<Add, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let model = asset_server.load_trenchbroom_model::<T>();
    commands.entity(add.entity).insert(SceneRoot(model));
}

pub(crate) fn setup_static_prop_with_convex_decomposition<T: QuakeClass>(
    add: On<Add, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = static_bundle::<T>(
        &asset_server,
        ColliderConstructor::ConvexDecompositionFromMesh,
    );
    commands.entity(add.entity).insert(bundle);
}

pub(crate) fn setup_dynamic_prop_with_convex_hull<T: QuakeClass>(
    add: On<Add, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = dynamic_bundle::<T>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands.entity(add.entity).insert(bundle);
}

pub(crate) fn dynamic_bundle<T: QuakeClass>(
    asset_server: &AssetServer,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = asset_server.load_trenchbroom_model::<T>();
    (
        ColliderConstructorHierarchy::new(constructor)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
        RigidBody::Dynamic,
        SceneRoot(model),
    )
}

pub(crate) fn static_bundle<T: QuakeClass>(
    asset_server: &AssetServer,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = asset_server.load_trenchbroom_model::<T>();
    (
        ColliderConstructorHierarchy::new(constructor).with_default_layers(CollisionLayers::new(
            CollisionLayer::Default,
            LayerMask::ALL,
        )),
        RigidBody::Static,
        SceneRoot(model),
    )
}
