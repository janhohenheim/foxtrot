use crate::third_party::avian3d::CollisionLayer;
use crate::third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _;
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
    world.commands().entity(entity).insert(bundle);
}

pub(crate) fn dynamic_bundle<T: QuakeClass>(world: &DeferredWorld) -> impl Bundle {
    let model = {
        let assets = world.resource::<AssetServer>();
        let model = assets.load(T::scene_path());
        if !assets.is_loaded_with_dependencies(model.id()) {
            warn!(
                "Model \"{}\" was not preloaded and will load during gameplay. Did you forget to add it to the `LevelAssets` resource?",
                T::scene_path()
            );
        }
        model
    };
    (
        TrenchBroomGltfRotationFix,
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
