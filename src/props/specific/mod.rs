//! Setup methods for specific props that require additional logic or need to be initialized with fine-tuned constants.

use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

use crate::{
    props::generic::dynamic_bundle,
    third_party::{
        avian3d::CollisionLayer,
        bevy_trenchbroom::{LoadTrenchbroomModel as _, fix_gltf_rotation},
    },
};
pub(crate) use burning_logs::*;

use super::{Chair, Crate, LampSitting, effects::insert_not_shadow_caster};

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

pub(crate) fn setup_crate(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let model = world.load_trenchbroom_model::<Crate>();
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert((
            TransformInterpolation,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
                .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                .with_default_density(1600.0),
            RigidBody::Dynamic,
            SceneRoot(model),
        ));
}

pub(crate) fn setup_lamp_sitting(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle =
        dynamic_bundle::<LampSitting>(&world, ColliderConstructor::ConvexDecompositionFromMesh);
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert(bundle)
        .with_child((
            Transform::from_xyz(0.0, 0.2, 0.0),
            PointLight {
                color: Color::srgb(1.0, 0.7, 0.4),
                intensity: 40_000.0,
                radius: 0.2,
                shadows_enabled: true,
                ..default()
            },
        ))
        .observe(insert_not_shadow_caster);
}
