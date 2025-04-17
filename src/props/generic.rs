use crate::third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _;
use crate::third_party::{avian3d::CollisionLayer, bevy_trenchbroom::fix_gltf_rotation};
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_tnua::TnuaNotPlatform;
use bevy_trenchbroom::{class::QuakeClass, prelude::*};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_dynamic_prop<T: QuakeClass>(
    mut world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    if world.is_scene_world() {
        return;
    }

    let bundle = dynamic_bundle::<T>(&world);
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert(bundle);
}

pub(crate) fn setup_static_prop<T: QuakeClass>(
    mut world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    if world.is_scene_world() {
        return;
    }

    let bundle = static_bundle::<T>(&world);
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert(bundle);
}

pub(crate) fn dynamic_bundle<T: QuakeClass>(world: &DeferredWorld) -> impl Bundle {
    let model = load_model::<T>(world);
    (
        TransformInterpolation,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
        RigidBody::Dynamic,
        TnuaNotPlatform,
        SceneRoot(model),
    )
}

pub(crate) fn static_bundle<T: QuakeClass>(world: &DeferredWorld) -> impl Bundle {
    let model = load_model::<T>(world);
    (
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_layers(CollisionLayers::new(
                CollisionLayer::Default,
                LayerMask::ALL,
            )),
        RigidBody::Static,
        SceneRoot(model),
    )
}

fn load_model<T: QuakeClass>(world: &DeferredWorld) -> Handle<Scene> {
    world.resource::<AssetServer>().load(T::scene_path())
}
