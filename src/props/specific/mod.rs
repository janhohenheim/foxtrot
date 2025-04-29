//! Setup methods for specific props that require additional logic or need to be initialized with fine-tuned constants.

use crate::{
    props::generic::dynamic_bundle,
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel as _},
};
use avian_pickup::prop::{PreferredPickupDistanceOverride, PreferredPickupRotation};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{Character, prelude::*};

use super::{Chair, Crate, LampSitting, effects::prepare_light_mesh};

mod burning_logs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(burning_logs::plugin);
    app.add_observer(setup_lamp_sitting);
    app.add_observer(setup_crate);
    app.add_observer(setup_chair);
}

pub(crate) fn setup_chair(
    trigger: Trigger<OnAdd, Chair>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let model = asset_server.load_trenchbroom_model::<Chair>();
    commands.entity(trigger.target()).insert(Character3dBundle {
        character: Character::default(),
        settings: CharacterSettings { radius: 0.4 },
        archipelago_ref: ArchipelagoRef3d::new(*archipelago),
    });

    commands.entity(trigger.target()).insert((
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

pub(crate) fn setup_crate(
    trigger: Trigger<OnAdd, Crate>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let model = asset_server.load_trenchbroom_model::<Crate>();
    commands.entity(trigger.target()).insert(Character3dBundle {
        character: Character::default(),
        settings: CharacterSettings { radius: 0.5 },
        archipelago_ref: ArchipelagoRef3d::new(*archipelago),
    });
    commands.entity(trigger.target()).insert((
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

pub(crate) fn setup_lamp_sitting(
    trigger: Trigger<OnAdd, LampSitting>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = dynamic_bundle::<LampSitting>(
        &asset_server,
        ColliderConstructor::ConvexDecompositionFromMesh,
    );
    commands
        .entity(trigger.target())
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
