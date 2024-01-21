use crate::file_system_interaction::game_state_serialization::{GameLoadRequest, GameSaveRequest};
use crate::file_system_interaction::level_serialization::{WorldLoadRequest, WorldSaveRequest};
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::camera::ForceCursorGrabMode;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_editor_pls::editor_window::EditorWindow;
use bevy_editor_pls::{
    editor::{Editor, EditorEvent},
    AddEditorWindow,
};
use bevy_egui::egui;
use bevy_egui::egui::ScrollArea;
use bevy_mod_sysfail::*;
use bevy_rapier3d::prelude::*;

use serde::{Deserialize, Serialize};
use spew::prelude::*;
use strum::IntoEnumIterator;

pub(crate) fn dev_editor_plugin(app: &mut App) {
    app.init_resource::<DevEditorState>()
        .add_editor_window::<DevEditorWindow>()
        .add_systems(
            Update,
            (handle_debug_render, set_cursor_grab_mode).run_if(in_state(GameState::Playing)),
        );
}

pub(crate) struct DevEditorWindow;

impl EditorWindow for DevEditorWindow {
    type State = DevEditorState;
    const NAME: &'static str = "Foxtrot Dev";
    const DEFAULT_SIZE: (f32, f32) = (200., 150.);
    fn ui(
        world: &mut World,
        mut cx: bevy_editor_pls::editor_window::EditorWindowContext,
        ui: &mut egui::Ui,
    ) {
        let state = cx
            .state_mut::<DevEditorWindow>()
            .expect("Failed to get dev window state");

        state.open = true;
        ui.heading("Debug Rendering");
        ui.checkbox(&mut state.collider_render_enabled, "Colliders");
        ui.checkbox(&mut state.navmesh_render_enabled, "Navmeshes");
        ui.separator();

        ui.heading("Scene Control");
        ui.horizontal(|ui| {
            ui.label("Level name: ");
            ui.text_edit_singleline(&mut state.level_name);
        });

        ui.add_enabled_ui(!state.level_name.is_empty(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    world.send_event(WorldSaveRequest {
                        filename: state.level_name.clone(),
                    })
                }
                if ui.button("Load").clicked() {
                    world.send_event(WorldLoadRequest {
                        filename: state.level_name.clone(),
                    });
                    // Make sure the player is spawned after the level
                    world.send_event(
                        SpawnEvent::with_data(
                            GameObject::Player,
                            Transform::from_translation((0., 1.5, 0.).into()),
                        )
                        .delay_frames(2),
                    );
                }
            });
        });
        ui.horizontal(|ui| {
            ui.label("Save name: ");
            ui.text_edit_singleline(&mut state.save_name);
        });

        ui.horizontal(|ui| {
            let filename = (!state.save_name.is_empty()).then(|| state.save_name.clone());
            if ui.button("Save").clicked() {
                world.send_event(GameSaveRequest {
                    filename: filename.clone(),
                })
            }
            if ui.button("Load").clicked() {
                world.send_event(GameLoadRequest { filename });
            }
        });

        ui.add_space(10.);
        ui.label("Spawning");
        if ui.button("Spawn").clicked() {
            world.send_event(SpawnEvent::with_data(
                state.spawn_item,
                Transform::default(),
            ));
        }

        ui.add_space(3.);

        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for item in GameObject::iter() {
                        ui.radio_value(&mut state.spawn_item, item, format!("{item:?}"));
                    }
                });
            });
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct DevEditorState {
    pub(crate) open: bool,
    pub(crate) level_name: String,
    pub(crate) save_name: String,
    pub(crate) spawn_item: GameObject,
    pub(crate) collider_render_enabled: bool,
    pub(crate) navmesh_render_enabled: bool,
}

impl Default for DevEditorState {
    fn default() -> Self {
        Self {
            level_name: "old_town".to_owned(),
            save_name: default(),
            spawn_item: default(),
            collider_render_enabled: false,
            navmesh_render_enabled: false,
            open: false,
        }
    }
}

#[sysfail(log(level = "error"))]
fn handle_debug_render(
    state: Res<Editor>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) -> Result<()> {
    debug_render_context.enabled = state
        .window_state::<DevEditorWindow>()
        .context("Failed to read dev window state")?
        .collider_render_enabled;
    Ok(())
}

fn set_cursor_grab_mode(
    mut events: EventReader<EditorEvent>,
    mut force_cursor_grab: ResMut<ForceCursorGrabMode>,
) {
    for event in events.read() {
        if let EditorEvent::Toggle { now_active } = event {
            if *now_active {
                force_cursor_grab.0 = Some(CursorGrabMode::None);
            } else {
                force_cursor_grab.0 = None;
            }
        }
    }
}
