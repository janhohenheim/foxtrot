use crate::file_system_interaction::config::GameConfig;
use crate::GameState;
use anyhow::Result;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_egui::egui::ProgressBar;
use bevy_egui::{egui, EguiContexts};
use bevy_kira_audio::AudioSource;
use bevy_mod_sysfail::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

pub(crate) fn loading_plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<GameConfig>::new(&["game.toml"]))
        .add_plugins(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<GltfAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<ConfigAssets>(),
        )
        .add_systems(Update, show_progress.run_if(in_state(GameState::Loading)))
        .add_systems(Update, update_config);
}

// the following asset collections will be loaded during the State `GameState::InitialLoading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub(crate) walking: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct GltfAssets {
    #[asset(path = "scenes/level.glb")]
    pub(crate) level: Handle<Gltf>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct TextureAssets {
    #[asset(path = "textures/stone_alley_2.jpg")]
    pub(crate) glowy_interior: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ConfigAssets {
    #[allow(dead_code)]
    #[asset(path = "config/config.game.toml")]
    pub(crate) game: Handle<GameConfig>,
}

fn show_progress(
    progress: Option<Res<ProgressCounter>>,
    mut egui_contexts: EguiContexts,
    mut last_done: Local<u32>,
    audio_assets: Option<Res<AudioAssets>>,
    gltf_assets: Option<Res<GltfAssets>>,
    texture_assets: Option<Res<TextureAssets>>,
    config_assets: Option<Res<ConfigAssets>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
        }

        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
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
                    ui.checkbox(&mut gltf_assets.is_some(), "Models");
                    ui.checkbox(&mut texture_assets.is_some(), "Textures");
                    ui.checkbox(&mut config_assets.is_some(), "Config");
                });
            });
        });
    }
}

#[sysfail(log(level = "error"))]
fn update_config(
    mut commands: Commands,
    config: Res<Assets<GameConfig>>,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_config").entered();
    for event in config_asset_events.read() {
        match event {
            AssetEvent::Modified { id } | AssetEvent::LoadedWithDependencies { id } => {
                // Guaranteed by Bevy to not fail
                let config = config.get(*id).unwrap();
                commands.insert_resource(config.clone());
            }
            _ => {}
        }
    }
    Ok(())
}
