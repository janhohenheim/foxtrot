use crate::actions::{Actions, ActionsFrozen};
use crate::spawning::{GameObject, ParentChangeEvent, SpawnEvent as SpawnRequestEvent};
use crate::world_serialization::{LoadRequest, SaveRequest};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::ScrollArea;
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
    entity_name: String,
    spawn_item: GameObject,
}

impl Default for SceneEditorState {
    fn default() -> Self {
        Self {
            save_name: "demo".to_owned(),
            active: default(),
            spawn_name: default(),
            parent_name: default(),
            entity_name: default(),
            spawn_item: default(),
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
    mut save_events: EventWriter<SaveRequest>,
    mut load_events: EventWriter<LoadRequest>,
    mut parenting_events: EventWriter<ParentChangeEvent>,
    mut state: ResMut<SceneEditorState>,
) {
    if !state.active {
        return;
    }
    const HEIGHT: f32 = 200.;
    const WIDTH: f32 = 150.;

    egui::Window::new("Scene Editor")
        .default_size(egui::Vec2::new(HEIGHT, WIDTH))
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Scene Control");
            ui.horizontal(|ui| {
                ui.label("Save name: ");
                ui.text_edit_singleline(&mut state.save_name);
            });

            ui.add_enabled_ui(!state.save_name.is_empty(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save_events.send(SaveRequest {
                            filename: state.save_name.clone(),
                        })
                    }
                    if ui.button("Load").clicked() {
                        load_events.send(LoadRequest {
                            filename: state.save_name.clone(),
                        })
                    }
                });
            });
            ui.separator();
            ui.heading("Entity Control");
            ui.horizontal(|ui| {
                ui.label("Entity:");
                ui.text_edit_singleline(&mut state.entity_name);
            });
            ui.horizontal(|ui| {
                ui.label("(New) Parent:");
                ui.text_edit_singleline(&mut state.parent_name);
            });
            let has_entity = !state.entity_name.is_empty();
            let has_valid_parent = !state.parent_name.is_empty()
                && has_entity
                && state.entity_name != state.parent_name;
            ui.horizontal(|ui| {
                ui.add_enabled_ui(has_valid_parent, |ui| {
                    if ui.button("Set parent").clicked() {
                        parenting_events.send(ParentChangeEvent {
                            name: state.entity_name.clone().into(),
                            new_parent: state.parent_name.clone().into(),
                        });
                        state.entity_name = default();
                        state.parent_name = default();
                    }
                });
                ui.add_enabled_ui(has_entity, |ui| {
                    if ui.button("Duplicate").clicked() {
                        state.entity_name = default();
                        state.parent_name = default();
                    }
                });
                ui.add_enabled_ui(has_entity, |ui| {
                    if ui.button("Spawn").clicked() {
                        let name = state.spawn_name.clone();
                        let name = (!name.is_empty()).then(|| name.into());

                        let parent = state.parent_name.clone();
                        let parent = (!parent.is_empty()).then(|| parent.into());
                        spawn_events.send(SpawnEvent {
                            object: state.spawn_item,
                            name,
                            parent,
                        });
                        state.entity_name = default();
                        state.parent_name = default();
                    }
                });
            });

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
