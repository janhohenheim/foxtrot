use std::f32::consts::TAU;

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
    println!("Spawning lamp shaded");
    world.commands().queue(move |world: &mut World| {
        world.resource_scope::<AssetServer, ()>(move |world, asset_server| {
            let bundle = quake_bundle::<LampShaded>(
                &asset_server,
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
    });
}
