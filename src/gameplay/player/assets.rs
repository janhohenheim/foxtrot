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
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load(format!("{}#Scene0", Player::model_path())),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
