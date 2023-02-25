use crate::file_system_interaction::config::GameConfig;
use crate::file_system_interaction::level_serialization::SerializedLevel;
use crate::world_interaction::dialog::Dialog;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_egui::egui::ProgressBar;
use bevy_egui::{egui, EguiContext};
use bevy_kira_audio::AudioSource;
use iyes_progress::{ProgressCounter, ProgressPlugin};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<SerializedLevel>::new(&["lvl.ron"]))
            .add_plugin(RonAssetPlugin::<Dialog>::new(&["dlg.ron"]))
            .add_plugin(TomlAssetPlugin::<GameConfig>::new(&["game.toml"]))
            .add_plugin(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .with_collection::<AudioAssets>()
                    .with_collection::<SceneAssets>()
                    .with_collection::<AnimationAssets>()
                    .with_collection::<LevelAssets>()
                    .with_collection::<DialogAssets>()
                    .with_collection::<TextureAssets>()
                    .with_collection::<ConfigAssets>(),
            )
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(show_progress));
    }
}

// the following asset collections will be loaded during the State `GameState::InitialLoading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub walking: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "scenes/Fox.glb#Scene0")]
    pub character: Handle<Scene>,
    #[asset(path = "scenes/old_town.glb#Scene0")]
    pub level: Handle<Scene>,
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
    #[cfg_attr(feature = "native", asset(path = "levels", collection(typed, mapped)))]
    #[cfg_attr(
        feature = "wasm",
        asset(paths("levels/old_town.lvl.ron"), collection(typed, mapped))
    )]
    pub levels: HashMap<String, Handle<SerializedLevel>>,
}

#[derive(AssetCollection, Resource)]
pub struct DialogAssets {
    #[cfg_attr(feature = "native", asset(path = "dialogs", collection(typed, mapped)))]
    #[cfg_attr(
        feature = "wasm",
        asset(paths("dialogs/follower.dlg.ron"), collection(typed, mapped))
    )]
    pub dialogs: HashMap<String, Handle<Dialog>>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/stone_alley_2.jpg")]
    pub glowy_interior: Handle<Image>,
    #[asset(path = "textures/sky.jpg")]
    pub sky: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ConfigAssets {
    #[asset(path = "config/config.game.toml")]
    pub game: Handle<GameConfig>,
}

fn show_progress(
    progress: Option<Res<ProgressCounter>>,
    mut egui_context: ResMut<EguiContext>,
    mut last_done: Local<u32>,
    audio_assets: Option<Res<AudioAssets>>,
    scene_assets: Option<Res<SceneAssets>>,
    animation_assets: Option<Res<AnimationAssets>>,
    level_assets: Option<Res<LevelAssets>>,
    dialog_assets: Option<Res<DialogAssets>>,
    texture_assets: Option<Res<TextureAssets>>,
    config_assets: Option<Res<ConfigAssets>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
        }

        egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Loading");
                ui.label("Loading assets...");
                ui.add(
                    ProgressBar::new(progress.done as f32 / progress.total as f32).animate(true),
                );
                ui.add_space(100.0);
                ui.add_enabled_ui(false, |ui| {
                    ui.checkbox(&mut audio_assets.is_some(), "Audio");
                    ui.checkbox(&mut scene_assets.is_some(), "Scenes");
                    ui.checkbox(&mut animation_assets.is_some(), "Animations");
                    ui.checkbox(&mut level_assets.is_some(), "Levels");
                    ui.checkbox(&mut dialog_assets.is_some(), "Dialogs");
                    ui.checkbox(&mut texture_assets.is_some(), "Textures");
                    ui.checkbox(&mut config_assets.is_some(), "Config");
                });
            });
        });
    }
}
