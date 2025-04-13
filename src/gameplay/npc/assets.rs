use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource, third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

use super::Npc;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<NpcAssets>();
    app.load_resource::<NpcAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct NpcAssets {
    #[dependency]
    pub(crate) model: Handle<Scene>,
    #[dependency]
    pub(crate) idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) walk_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) run_animation: Handle<AnimationClip>,
}

impl FromWorld for NpcAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let load_animation = |name: &str| -> Handle<AnimationClip> {
            assets.load(format!("{}#Animation{}", Npc::model_path(), name))
        };
        Self {
            model: assets.load(format!("{}#Scene0", Npc::model_path())),
            run_animation: load_animation("0"),
            idle_animation: load_animation("1"),
            walk_animation: load_animation("2"),
        }
    }
}
