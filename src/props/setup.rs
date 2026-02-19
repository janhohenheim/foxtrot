//! Utility functions for creating regular props that don't have any special properties.
//! A *dynamic* prop in the context of this file is a prop that is influenced by physics,
//! while a *static* prop is unmovable terrain.

use std::any::type_name;

use crate::third_party::avian3d::CollisionLayer;
use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel as _;
use avian3d::prelude::*;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

use bevy_trenchbroom::class::QuakeClass;

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_prop<T: QuakeClass>(
    rigid_body: RigidBody,
    collider: ColliderConstructor,
) -> impl FnOnce(DeferredWorld, HookContext) {
    move |mut world, ctx| {
        println!("Setting up {}", type_name::<T>());
        world.commands().queue(move |world: &mut World| {
            world.resource_scope::<AssetServer, ()>(move |world, asset_server| {
                world.entity_mut(ctx.entity).insert(quake_bundle::<T>(
                    &asset_server,
                    rigid_body,
                    collider,
                ));
            })
        });
    }
}

pub(crate) fn setup_nonphysical_prop<T: QuakeClass>(mut world: DeferredWorld, ctx: HookContext) {
    world.commands().queue(move |world: &mut World| {
        world.resource_scope::<AssetServer, ()>(move |world, asset_server| {
            let model = asset_server.load_trenchbroom_model::<T>();
            world.entity_mut(ctx.entity).insert(SceneRoot(model));
        });
    });
}

pub(crate) fn quake_bundle<T: QuakeClass>(
    asset_server: &AssetServer,
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
