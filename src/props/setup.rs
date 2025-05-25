//! Utility functions for creating regular props that don't have any special properties.
//! A *dynamic* prop in the context of this file is a prop that is influenced by physics,
//! while a *static* prop is unmovable terrain.

use crate::third_party::avian3d::CollisionLayer;
use crate::third_party::bevy_landmass::NavMeshAffectorParent;
use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel as _;
use avian3d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::TnuaNotPlatform;
use bevy_trenchbroom::class::QuakeClass;

pub(super) fn plugin(_app: &mut App) {}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn setup_static_prop_with_convex_hull<T: QuakeClass>(
    trigger: Trigger<OnAdd, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = static_bundle::<T>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands.entity(trigger.target()).insert(bundle);
}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn setup_nonphysical_prop<T: QuakeClass>(
    trigger: Trigger<OnAdd, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let model = asset_server.load_trenchbroom_model::<T>();
    commands.entity(trigger.target()).insert(SceneRoot(model));
}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn setup_static_prop_with_convex_decomposition<T: QuakeClass>(
    trigger: Trigger<OnAdd, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = static_bundle::<T>(
        &asset_server,
        ColliderConstructor::ConvexDecompositionFromMesh,
    );
    commands.entity(trigger.target()).insert(bundle);
}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn setup_dynamic_prop_with_convex_hull<T: QuakeClass>(
    trigger: Trigger<OnAdd, T>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = dynamic_bundle::<T>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands.entity(trigger.target()).insert(bundle);
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
        // `TnuaNotPlatform` ensures that the character controller will not try to walk on the prop.
        // Removing this will make it so that throwing a prop at a controller sends them flying so that they stand on top of it.
        TnuaNotPlatform,
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
        NavMeshAffectorParent,
    )
}
