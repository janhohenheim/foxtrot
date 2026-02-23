use avian3d::prelude::*;
use bevy::{
    app::Propagate,
    asset::io::embedded::GetAssetServer as _,
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
    app.load_asset::<Gltf>(LampWallElectric::model_path());
}

#[point_class(
    base(Transform, Visibility),
    model(
        "models/darkmod/lights/non-extinguishable/lamp_wall_electric_01/lamp_wall_electric_01.gltf"
    ),
    classname("light_lamp_wall_electric")
)]
#[component(on_add = setup_lamp_wall_electric)]
pub(crate) struct LampWallElectric;

fn setup_lamp_wall_electric(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    world.commands().queue(move |world: &mut World| {
        let asset_server = world.get_asset_server().clone();
        let bundle = quake_bundle::<LampWallElectric>(
            asset_server,
            RigidBody::Static,
            ColliderConstructor::ConvexHullFromMesh,
        );
        world
            .entity_mut(ctx.entity)
            .insert((bundle, NotShadowCaster, Propagate(NotShadowCaster)))
            .with_child((
                Transform::from_xyz(0.0, -0.08, -0.35),
                PointLight {
                    color: Color::srgb_u8(232, 199, 176),
                    intensity: 40_000.0,
                    radius: 0.05,
                    range: 20.0,
                    shadows_enabled: true,
                    ..default()
                },
            ));
    });
}
