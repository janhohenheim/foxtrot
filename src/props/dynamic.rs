use crate::third_party::avian3d::CollisionLayer;
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_tnua::TnuaNotPlatform;
use bevy_trenchbroom::{class::QuakeClass, prelude::*};

use super::loading::LoadModel as _;

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_dynamic_prop<T: QuakeClass>(
    mut world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    if world.is_scene_world() {
        return;
    }
    let model = world.resource::<AssetServer>().load_model::<T>();
    world
        .commands()
        .entity(entity)
        .insert((dynamic_bundle(), SceneRoot(model)));
}

pub(crate) fn dynamic_bundle() -> impl Bundle {
    (
        TrenchBroomGltfRotationFix,
        TransformInterpolation,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
        RigidBody::Dynamic,
        TnuaNotPlatform,
    )
}
