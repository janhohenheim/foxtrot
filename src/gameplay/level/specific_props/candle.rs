use crate::gameplay::level::{
    assets::LevelAssets,
    dynamic_props::{collider_constructor_hierarchy, dynamic_bundle},
    props::{Candle, CandleUnlit},
};
use avian_pickup::prop::HeldProp;
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    scene::SceneInstance,
};
use bevy_tnua::TnuaNotPlatform;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_candle(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let model = world.resource::<LevelAssets>().model_for_class::<Candle>();
    world
        .commands()
        .entity(entity)
        .insert((
            TnuaNotPlatform,
            TransformInterpolation,
            TrenchBroomGltfRotationFix,
            RigidBody::Dynamic,
        ))
        .with_child((collider_constructor_hierarchy(), SceneRoot(model)))
        .observe(extinguish_candle);
}

fn extinguish_candle(
    trigger: Trigger<OnAdd, HeldProp>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    let candle = trigger.entity();
    commands.entity(candle).despawn_descendants();
    commands.entity(candle).with_child((
        SceneRoot(level_assets.model_for_class::<CandleUnlit>()),
        collider_constructor_hierarchy(),
    ));
}
