use crate::gameplay::level::{
    assets::LevelAssets,
    dynamic_props::{CandleUnlit, dynamic_bundle},
    prop_util::create_prop,
};
use avian_pickup::prop::HeldProp;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

create_prop!(Candle, "models/candle/candle.gltf", on_add = setup_candle);

fn setup_candle(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let model = world.resource::<LevelAssets>().model_for_class::<Candle>();
    world
        .commands()
        .entity(entity)
        .insert(dynamic_bundle())
        .with_child(SceneRoot(model))
        .observe(extinguish_candle);
}

fn extinguish_candle(
    trigger: Trigger<OnAdd, HeldProp>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    let candle = trigger.entity();
    commands.entity(candle).despawn_descendants();
    commands
        .entity(candle)
        .insert(SceneRoot(level_assets.model_for_class::<CandleUnlit>()));
}
