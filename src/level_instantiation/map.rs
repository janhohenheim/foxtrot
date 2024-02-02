use crate::file_system_interaction::asset_loading::GltfAssets;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub(crate) fn map_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level)
        .add_systems(
            Update,
            show_loading_screen
                .run_if(not(any_with_component::<Player>()).and_then(in_state(GameState::Playing))),
        );
}

fn spawn_level(mut commands: Commands, models: Res<Assets<Gltf>>, gltf_assets: Res<GltfAssets>) {
    let my_gltf = models.get(&gltf_assets.level).unwrap();
    info!("spawning scene");
    commands.spawn((
        SceneBundle {
            scene: my_gltf.scenes[1].clone(),
            ..default()
        },
        Name::new("Level"),
    ));
}

fn show_loading_screen(mut egui_contexts: EguiContexts) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Loading");
            ui.label("Spawning level...");
            ui.add_space(10.0);
        });
    });
}
