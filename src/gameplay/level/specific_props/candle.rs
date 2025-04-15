use crate::gameplay::level::{
    assets::LevelAssets, dynamic_props::collider_constructor_hierarchy, props::Candle,
};
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
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
        .with_child((collider_constructor_hierarchy(), SceneRoot(model)));
}
