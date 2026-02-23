//! Utility functions for creating regular props that don't have any special properties.
//! A *dynamic* prop in the context of this file is a prop that is influenced by physics,
//! while a *static* prop is unmovable terrain.

use crate::third_party::avian3d::CollisionLayer;
use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel as _;
use avian3d::prelude::*;
use bevy::asset::io::embedded::GetAssetServer;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

use bevy_trenchbroom::class::QuakeClass;
use bevy_trenchbroom::util::IsSceneWorld as _;

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_prop<T: QuakeClass>(
    rigid_body: RigidBody,
    collider: ColliderConstructor,
) -> impl FnOnce(DeferredWorld, HookContext) {
    move |mut world, ctx| {
        if world.is_scene_world() {
            return;
        }
        world.commands().queue(move |world: &mut World| {
            let asset_server = world.get_asset_server().clone();
            world.entity_mut(ctx.entity).insert(quake_bundle::<T>(
                asset_server,
                rigid_body,
                collider,
            ));
        });
    }
}

pub(crate) fn setup_nonphysical_prop<T: QuakeClass>(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    world.commands().queue(move |world: &mut World| {
        let asset_server = world.get_asset_server().clone();
        let model = asset_server.load_trenchbroom_model::<T>();
        world.entity_mut(ctx.entity).insert(SceneRoot(model));
    });
}

pub(crate) fn quake_bundle<T: QuakeClass>(
    asset_server: AssetServer,
    rigid_body: RigidBody,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = asset_server.load_trenchbroom_model::<T>();
    (
        ColliderConstructorHierarchy::new(constructor)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
        rigid_body,
        SceneRoot(model),
    )
}
