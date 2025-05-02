//! Assets for the player.

use bevy::{asset::RenderAssetUsages, gltf::GltfLoaderSettings, prelude::*};
use bevy_shuffle_bag::ShuffleBag;

use crate::{
    asset_tracking::LoadResource, third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct PlayerAssets {
    #[dependency]
    pub(crate) _model: Handle<Scene>,
    #[dependency]
    pub(crate) throw_sound: Handle<AudioSource>,
    #[dependency]
    pub(crate) steps: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) jump_grunts: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) land_sounds: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) jump_start_sounds: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) a_pose_animation: Handle<AnimationClip>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let rng = &mut rand::thread_rng();
        Self {
            _model: assets.load_with_settings(
                Player::scene_path(),
                |settings: &mut GltfLoaderSettings| {
                    settings.load_meshes = RenderAssetUsages::RENDER_WORLD;
                    settings.load_materials = RenderAssetUsages::RENDER_WORLD;
                },
            ),
            throw_sound: assets.load("audio/sound_effects/throw.ogg"),
            steps: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_01.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_02.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_03.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_04.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_05.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_06.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_07.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_08.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_09.ogg"),
                ],
                rng,
            )
            .unwrap(),
            jump_grunts: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_1.ogg"),
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_2.ogg"),
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_3.ogg"),
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_4.ogg"),
                ],
                rng,
            )
            .unwrap(),
            land_sounds: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_01.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_02.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_03.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_04.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_05.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_06.ogg"),
                ],
                rng,
            )
            .unwrap(),
            jump_start_sounds: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_01.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_02.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_03.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_04.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_05.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_06.ogg"),
                ],
                rng,
            )
            .unwrap(),
            idle_animation: assets.load(Player::animation_path(9)),
            a_pose_animation: assets.load(Player::animation_path(5)),
        }
    }
}
