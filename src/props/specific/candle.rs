use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

use crate::props::{Candle, dynamic::dynamic_bundle, loading::LoadModel as _};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_candle(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let model = world.resource::<AssetServer>().load_model::<Candle>();
    world
        .commands()
        .entity(entity)
        .insert((dynamic_bundle(), SceneRoot(model)));
}
