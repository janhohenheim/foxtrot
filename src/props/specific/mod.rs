//! Setup methods for specific props that require additional logic or need to be initialized with fine-tuned constants.

use avian_pickup::prop::{PreferredPickupDistanceOverride, PreferredPickupRotation};
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
        bevy_landmass::insert_landmass_character,
        bevy_trenchbroom::{LoadTrenchbroomModel as _, fix_gltf_rotation},
    },
};
pub(crate) use burning_logs::*;

use super::{Chair, Crate, LampSitting, effects::prepare_light_mesh};

mod burning_logs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(burning_logs::plugin);
}

pub(crate) fn setup_chair(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    world
        .commands()
        .run_system_cached_with(insert_landmass_character, (entity, 0.4));

    let model = world.load_trenchbroom_model::<Chair>();
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert((
            // The chair has a fairly complex shape, so let's use a convex decomposition.
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh)
                .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                // Make the chair way more dense than the default, as it feels janky to be able to push it around easily.
                .with_default_density(10_000.0),
            TransformInterpolation,
            RigidBody::Dynamic,
            // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
            SceneRoot(model),
        ));
}

pub(crate) fn setup_crate(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    world
        .commands()
        .run_system_cached_with(insert_landmass_character, (entity, 0.5));
    let model = world.load_trenchbroom_model::<Crate>();
    world
        .commands()
        .entity(entity)
        .queue(fix_gltf_rotation)
        .insert((
            TransformInterpolation,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
                .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                .with_default_density(1_000.0),
            // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
            RigidBody::Dynamic,
            SceneRoot(model),
            // The prop should be held upright.
            PreferredPickupRotation(Quat::IDENTITY),
            // Holding a big crate right in your face looks bad, so
            // let's move the pickup distance a bit further away.
            PreferredPickupDistanceOverride(1.0),
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
        // The prop should be held upright.
        .insert((bundle, PreferredPickupRotation(Quat::IDENTITY)))
        // The lamp's origin is at the bottom of the lamp, so we need to offset the light a bit.
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
        .observe(prepare_light_mesh);
}
