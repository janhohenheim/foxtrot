//! Preload NPC assets.

use bevy::{asset::RenderAssetUsages, gltf::GltfLoaderSettings, prelude::*};
use bevy_shuffle_bag::ShuffleBag;

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
    pub(crate) _model: Handle<Scene>,
    #[dependency]
    pub(crate) idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) walk_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) run_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) steps: ShuffleBag<Handle<AudioSource>>,
}

impl FromWorld for NpcAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let rng = &mut rand::thread_rng();
        Self {
            _model: assets.load_with_settings(
                Npc::scene_path(),
                |settings: &mut GltfLoaderSettings| {
                    settings.load_meshes = RenderAssetUsages::RENDER_WORLD;
                    settings.load_materials = RenderAssetUsages::RENDER_WORLD;
                },
            ),
            run_animation: assets.load(Npc::animation_path(0)),
            idle_animation: assets.load(Npc::animation_path(1)),
            walk_animation: assets.load(Npc::animation_path(2)),
            steps: ShuffleBag::try_new(
                [
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
                rng,
            )
            .unwrap(),
        }
    }
}
