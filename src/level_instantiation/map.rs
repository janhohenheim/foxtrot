use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::{DelayedSpawnEvent, GameObject, SpawnEvent};
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(show_loading_screen),
            );
        #[cfg(feature = "wasm")]
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(show_wasm_loader));
    }
}

fn setup(
    mut commands: Commands,
    mut loader: EventWriter<WorldLoadRequest>,
    mut delayed_spawner: EventWriter<DelayedSpawnEvent>,
    current_level: Option<Res<CurrentLevel>>,
) {
    if current_level.is_some() {
        return;
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    loader.send(WorldLoadRequest {
        filename: "old_town".to_string(),
    });

    // Make sure the player is spawned after the level
    delayed_spawner.send(DelayedSpawnEvent {
        tick_delay: 2,
        event: SpawnEvent {
            object: GameObject::Player,
            transform: Transform::from_xyz(0., 1.5, 0.),
        },
    });
}

fn show_loading_screen(player_query: Query<&Player>, mut egui_context: ResMut<EguiContext>) {
    if player_query.iter().next().is_none() {
        egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Loading");
                ui.label("Spawning level...");
                ui.add_space(10.0);
                #[cfg(feature = "wasm")]
                ui.add_space(40.0); // Spinner from CSS (build/web/styles.css) goes here.
                #[cfg(feature = "wasm")]
                ui.label("This may take a while. Don't worry, your browser did not crash!");
            });
        });
    }
}

#[cfg(feature = "wasm")]
fn show_wasm_loader(player_query: Query<&Player>, mut egui_context: ResMut<EguiContext>) {
    let id = egui::Id::new("loading-screen-shown");
    let memory = &mut egui_context.ctx_mut().memory().data;
    match (memory.get_temp::<()>(id), player_query.iter().next()) {
        (None, None) => {
            loader::show_loader();
            memory.insert_temp(id, ());
        }
        (Some(_), Some(_)) => {
            loader::hide_loader();
            memory.remove::<()>(id);
        }
        _ => {}
    }
}

#[cfg(feature = "wasm")]
mod loader {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "
        export function show_loader() {
            document.querySelector('.loader').hidden = false;
        }
        export function hide_loader() {
            document.querySelector('.loader').hidden = true;
        }")]
    extern "C" {
        pub fn show_loader();

        pub fn hide_loader();
    }
}
