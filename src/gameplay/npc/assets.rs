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
    #[dependency]
    pub(crate) steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for NpcAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let load_animation = |name: &str| -> Handle<AnimationClip> {
            let file_path = Npc::file_path();
            assets.load(format!("{file_path}#Animation{name}"))
        };
        Self {
            model: assets.load(Npc::scene_path()),
            run_animation: load_animation("0"),
            idle_animation: load_animation("1"),
            walk_animation: load_animation("2"),
            steps: vec![
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_01.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_02.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_03.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_04.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_05.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_06.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_07.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_08.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_09.ogg"),
                assets.load("audio/sound_effects/run/Footsteps_Rock_Run_10.ogg"),
            ],
        }
    }
}
