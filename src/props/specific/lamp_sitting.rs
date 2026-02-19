use avian_pickup::prop::PreferredPickupRotation;
use avian3d::prelude::*;
use bevy::{
    app::{HierarchyPropagatePlugin, Propagate},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    light::NotShadowCaster,
    prelude::*,
};

use bevy_trenchbroom::prelude::*;

use crate::{
    asset_tracking::LoadResource as _, props::setup::quake_bundle,
    third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<HierarchyPropagatePlugin<NotShadowCaster>>() {
        app.add_plugins(HierarchyPropagatePlugin::<NotShadowCaster>::new(PostUpdate));
    }
    app.load_asset::<Gltf>(LampSitting::model_path());
}

#[point_class(
    base(Transform, Visibility),
    model(
        "models/darkmod/lights/non-extinguishable/round_lantern_sitting/round_lantern_sitting.gltf"
    )
)]
#[component(on_add = setup_lamp_sitting)]
pub(crate) struct LampSitting;

fn setup_lamp_sitting(mut world: DeferredWorld, ctx: HookContext) {
    println!("Spawning lamp sitting");
    world.commands().queue(move |world: &mut World| {
        world.resource_scope::<AssetServer, ()>(move |world, asset_server| {
            let bundle = quake_bundle::<LampSitting>(
                &asset_server,
                RigidBody::Dynamic,
                ColliderConstructor::ConvexDecompositionFromMesh,
            );

            world
                .entity_mut(ctx.entity)
                // The prop should be held upright.
                .insert((
                    bundle,
                    NotShadowCaster,
                    Propagate(NotShadowCaster),
                    PreferredPickupRotation(Quat::IDENTITY),
                ))
                // The lamp's origin is at the bottom of the lamp, so we need to offset the light a bit.
                .with_child((
                    Transform::from_xyz(0.0, 0.2, 0.0),
                    PointLight {
                        color: Color::srgb(1.0, 0.7, 0.4),
                        intensity: 40_000.0,
                        radius: 0.05,
                        shadows_enabled: true,
                        ..default()
                    },
                ));
        })
    });
}
