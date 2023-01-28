use crate::GameState;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<SceneAssets>()
                .with_collection::<AnimationAssets>()
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
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "scenes/Fox.glb")]
    /// Source: <https://opengameart.org/content/fox-and-shiba>
    pub character: Handle<Gltf>,
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
