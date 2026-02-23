use std::f32::consts::TAU;

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
    app.load_asset::<Gltf>(LampShaded::model_path());
}

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/lights/non-extinguishable/lamp_shaded03/lamp_shaded03.gltf"),
    classname("light_lamp_shaded03")
)]
#[component(on_add = setup_lamp_shaded)]
pub(crate) struct LampShaded;

fn setup_lamp_shaded(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    world.commands().queue(move |world: &mut World| {
        let asset_server = world.get_asset_server().clone();
        let bundle = quake_bundle::<LampShaded>(
            asset_server,
            RigidBody::Static,
            ColliderConstructor::ConvexHullFromMesh,
        );
        world
            .entity_mut(ctx.entity)
            .insert((bundle, NotShadowCaster, Propagate(NotShadowCaster)))
            .with_child((
                SpotLight {
                    color: Color::srgb_u8(232, 199, 176),
                    intensity: 800_000.0,
                    radius: 0.1,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.1, -0.25)
                    .with_rotation(Quat::from_rotation_x(TAU / 4.5)),
            ));
    });
}
