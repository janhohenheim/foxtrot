use avian3d::prelude::*;
use bevy::{
    asset::io::embedded::GetAssetServer as _,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_landmass::{Character, prelude::*};

use bevy_trenchbroom::prelude::*;

use crate::{
    asset_tracking::LoadResource as _,
    third_party::{
        avian3d::CollisionLayer,
        bevy_trenchbroom::{GetTrenchbroomModelPath as _, LoadTrenchbroomModel as _},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.load_asset::<Gltf>(Chair::model_path());
}

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/seating/wchair1.gltf")
)]
#[component(on_add = setup_chair)]
pub(crate) struct Chair;

fn setup_chair(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    world.commands().queue(move |world: &mut World| {
        let Ok(archipelago) = world
            .query_filtered::<Entity, With<Archipelago3d>>()
            .single(world)
        else {
            return;
        };

        let asset_server = world.get_asset_server().clone();
        let model = asset_server.load_trenchbroom_model::<Chair>();
        world.entity_mut(ctx.entity).insert(Character3dBundle {
            character: Character::default(),
            settings: CharacterSettings { radius: 0.4 },
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
        });

        world.entity_mut(ctx.entity).insert((
            // The chair has a fairly complex shape, so let's use a convex decomposition.
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh)
                .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                // Make the chair way more dense than the default, as it feels janky to be able to push it around easily.
                .with_default_density(10_000.0),
            RigidBody::Dynamic,
            // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
            SceneRoot(model),
        ));
    });
}
