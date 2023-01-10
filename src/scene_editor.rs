use crate::actions::{Actions, ActionsFrozen};
use crate::spawning::{GameObject, SpawnEvent as SpawnRequestEvent};
use crate::world_serialization::{LoadRequest, SaveRequest};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::{Align, ScrollArea};
use bevy_egui::{egui, EguiContext};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use strum::IntoEnumIterator;

pub struct SceneEditorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SceneEditorState {
    active: bool,
    save_name: String,
    spawn_name: String,
    parent_name: String,
}

impl Default for SceneEditorState {
    fn default() -> Self {
        Self {
            save_name: "demo".to_owned(),
            active: default(),
            spawn_name: default(),
            parent_name: default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SpawnEvent {
    object: GameObject,
    name: Option<Cow<'static, str>>,
    parent: Option<Cow<'static, str>>,
}

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        app.add_event::<SpawnEvent>()
            .init_resource::<SceneEditorState>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_toggle)
                    .with_system(show_editor)
                    .with_system(relay_spawn_requests),
            );

        let _ = app;
    }
}

fn handle_toggle(
    mut commands: Commands,
    actions: Res<Actions>,
    mut scene_editor_state: ResMut<SceneEditorState>,
) {
    if !actions.toggle_editor {
        return;
    }
    scene_editor_state.active = !scene_editor_state.active;

    if scene_editor_state.active {
        commands.init_resource::<ActionsFrozen>();
    } else {
        commands.remove_resource::<ActionsFrozen>();
    }
}

fn show_editor(
    mut egui_context: ResMut<EguiContext>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut save_writer: EventWriter<SaveRequest>,
    mut save_loader: EventWriter<LoadRequest>,
    mut editor_state: ResMut<SceneEditorState>,
) {
    if !editor_state.active {
        return;
    }
    egui::Window::new("Scene Editor")
        .fixed_size(egui::Vec2::new(150., 150.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Save name: ");
                ui.text_edit_singleline(&mut editor_state.save_name);
            });
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    save_writer.send(SaveRequest {
                        filename: editor_state.save_name.clone(),
                    })
                }
                if ui.button("Load").clicked() {
                    save_loader.send(LoadRequest {
                        filename: editor_state.save_name.clone(),
                    })
                }
            });

            ui.separator();
            ui.heading("Spawn object");
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(&mut editor_state.spawn_name);
            });
            ui.horizontal(|ui| {
                ui.label("Parent: ");
                ui.text_edit_singleline(&mut editor_state.parent_name);
            });
            ui.add_space(3.);

            ScrollArea::vertical()
                .max_height(100.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for item in GameObject::iter() {
                            let item_to_track = GameObject::Grass;
                            let track_item = false;
                            let item_to_track_align = Some(Align::Center);
                            ui.horizontal(|ui| {
                                let spawn_button = ui.button("⬛");
                                ui.label(format!("{item:?}"));
                                if track_item && item == item_to_track {
                                    spawn_button.scroll_to_me(item_to_track_align)
                                }
                                if spawn_button.clicked() {
                                    let name = editor_state.spawn_name.clone();
                                    editor_state.spawn_name = default();
                                    let name = (!name.is_empty()).then(|| name.into());

                                    let parent = editor_state.parent_name.clone();
                                    editor_state.parent_name = default();
                                    let parent = (!parent.is_empty()).then(|| parent.into());
                                    spawn_events.send(SpawnEvent {
                                        object: item,
                                        name,
                                        parent,
                                    });
                                }
                            });
                        }
                    });
                });
        });
}

fn relay_spawn_requests(
    mut spawn_requests: EventReader<SpawnEvent>,
    mut spawn_requester: EventWriter<SpawnRequestEvent>,
) {
    for object in spawn_requests.iter() {
        spawn_requester.send(SpawnRequestEvent {
            object: object.object,
            transform: Transform::default(),
            parent: object.parent.clone(),
            name: object.name.clone(),
        });
    }
}
