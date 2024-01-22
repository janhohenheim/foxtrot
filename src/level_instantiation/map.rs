use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use spew::prelude::*;

pub(crate) fn map_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing),
        setup.run_if(not(resource_exists::<CurrentLevel>())),
    )
    .add_systems(
        Update,
        show_loading_screen
            .run_if(not(any_with_component::<Player>()))
            .run_if(in_state(GameState::Playing)),
    );
}

fn setup(
    mut commands: Commands,
    mut loader: EventWriter<WorldLoadRequest>,
    mut delayed_spawner: EventWriter<SpawnEvent<GameObject, Transform>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    loader.send(WorldLoadRequest {
        filename: "old_town".to_string(),
    });

    // Make sure the player is spawned after the level
    delayed_spawner.send(
        SpawnEvent::with_data(GameObject::Player, Transform::from_xyz(0., 1.5, 0.)).delay_frames(2),
    );
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
