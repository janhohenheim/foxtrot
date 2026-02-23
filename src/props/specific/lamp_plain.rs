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
    app.load_asset::<Gltf>(LampPlain::model_path());
}

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/lights/non-extinguishable/electric_plain1_unattached.gltf"),
    classname("light_lamp_plain")
)]
#[component(on_add = setup_lamp_wall_electric)]
struct LampPlain {
    color: Color,
    intensity: f32,
}

impl Default for LampPlain {
    fn default() -> Self {
        Self {
            color: Color::srgb_u8(180, 180, 232),
            intensity: 13_000.0,
        }
    }
}

fn setup_lamp_wall_electric(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    world.commands().queue(move |world: &mut World| {
        let asset_server = world.get_asset_server().clone();
        let &LampPlain { color, intensity } = world
            .query::<&LampPlain>()
            .get(world, ctx.entity)
            .expect("Component `LampPlain` should exist");

        let bundle = quake_bundle::<LampPlain>(
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
                    color,
                    intensity,
                    radius: 0.05,
                    range: 20.0,
                    shadows_enabled: true,
                    ..default()
                },
            ));
    });
}
