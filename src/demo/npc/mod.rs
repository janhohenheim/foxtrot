use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel;

pub(super) fn plugin(_app: &mut App) {}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/fox/Fox.gltf")]
#[component(on_add = Self::on_add)]
pub(crate) struct Npc;

impl Npc {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        if world.is_scene_world() {
            return;
        }
        let model = world
            .resource::<AssetServer>()
            .load_trenchbroom_model::<Self>();
        world.commands().entity(entity).insert(SceneRoot(model));
    }
}
