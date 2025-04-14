use bevy::prelude::*;

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
    pub(crate) model: Handle<Scene>,
    #[dependency]
    pub(crate) steps: Vec<Handle<AudioSource>>,
    #[dependency]
    pub(crate) jump_grunts: Vec<Handle<AudioSource>>,
    #[dependency]
    pub(crate) idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) a_pose_animation: Handle<AnimationClip>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let load_animation = |name: &str| -> Handle<AnimationClip> {
            assets.load(format!("{}#Animation{}", Player::model_path(), name))
        };
        Self {
            model: assets.load(format!("{}#Scene0", Player::model_path())),
            steps: vec![
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
            jump_grunts: vec![
                assets.load("audio/sound_effects/jump_grunt/jump_grunt_1.ogg"),
                assets.load("audio/sound_effects/jump_grunt/jump_grunt_2.ogg"),
                assets.load("audio/sound_effects/jump_grunt/jump_grunt_3.ogg"),
                assets.load("audio/sound_effects/jump_grunt/jump_grunt_4.ogg"),
            ],
            idle_animation: load_animation("9"),
            a_pose_animation: load_animation("5"),
        }
    }
}
