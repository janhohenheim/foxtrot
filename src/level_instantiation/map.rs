use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use spew::prelude::*;

pub(crate) fn map_plugin(app: &mut App) {
    app.add_system(
        setup
            .run_if(not(resource_exists::<CurrentLevel>()))
            .in_schedule(OnEnter(GameState::Playing)),
    )
    .add_system(
        show_loading_screen
            .run_if(not(any_with_component::<Player>()))
            .in_set(OnUpdate(GameState::Playing)),
    );

    #[cfg(target_arch = "wasm32")]
    app.add_system(show_wasm_loader.in_set(OnUpdate(GameState::Playing)));
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
            #[cfg(target_arch = "wasm32")]
            ui.add_space(40.0); // Spinner from CSS (build/web/styles.css) goes here.
            #[cfg(target_arch = "wasm32")]
            ui.label("This may take a while. Don't worry, your browser did not crash!");
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn show_wasm_loader(player_query: Query<&Player>, mut egui_contexts: EguiContexts) {
    let id = egui::Id::new("loading-screen-shown");
    egui_contexts.ctx_mut().memory_mut(|memory| {
        let memory = &mut memory.data;
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
    });
}

#[cfg(target_arch = "wasm32")]
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
        pub(crate) fn show_loader();

        pub(crate) fn hide_loader();
    }
}
