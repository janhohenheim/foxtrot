use crate::file_system_interaction::level_serialization::SerializedLevel;
use crate::world_interaction::dialog::Dialog;
use crate::GameState;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<SerializedLevel>::new(&["lvl.ron"]))
            .add_plugin(RonAssetPlugin::<Dialog>::new(&["dlg.ron"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .with_collection::<FontAssets>()
                    .with_collection::<AudioAssets>()
                    .with_collection::<SceneAssets>()
                    .with_collection::<AnimationAssets>()
                    .with_collection::<LevelAssets>()
                    .with_collection::<DialogAssets>()
                    .with_collection::<TextureAssets>()
                    .continue_to_state(GameState::Menu),
            );
    }
}

// the following asset collections will be loaded during the State `GameState::InitialLoading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub walking: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "scenes/Fox.glb")]
    pub character: Handle<Gltf>,
    #[asset(path = "scenes/old_town.glb")]
    pub level: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct AnimationAssets {
    #[asset(path = "scenes/Fox.glb#Animation0")]
    pub character_idle: Handle<AnimationClip>,
    #[asset(path = "scenes/Fox.glb#Animation1")]
    pub character_walking: Handle<AnimationClip>,
    #[asset(path = "scenes/Fox.glb#Animation2")]
    pub character_running: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    // Not simply linking to "levels/" because of <https://github.com/NiklasEi/bevy_asset_loader/issues/104>
    #[asset(paths("levels/old_town.lvl.ron"), collection(typed, mapped))]
    pub levels: HashMap<String, Handle<SerializedLevel>>,
}

#[derive(AssetCollection, Resource)]
pub struct DialogAssets {
    // Not simply linking to "levels/" because of <https://github.com/NiklasEi/bevy_asset_loader/issues/104>
    #[asset(paths("dialogs/follower.dlg.ron"), collection(typed, mapped))]
    pub dialogs: HashMap<String, Handle<Dialog>>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/stone_alley_2.jpg")]
    pub glowy_interior: Handle<Image>,
}
