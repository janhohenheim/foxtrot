use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{Character, prelude::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_trenchbroom::prelude::*;

use crate::third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel as _};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_chair);
    app.register_type::<Chair>();
}

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/furniture/seating/wchair1.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct Chair;

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_chair(
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
        RigidBody::Dynamic,
        // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
        SceneRoot(model),
    ));
}
