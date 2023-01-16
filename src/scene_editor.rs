use crate::actions::{Actions, ActionsFrozen};
use crate::saving::{GameLoadRequest, GameSaveRequest};
use crate::spawning::{
    DelayedSpawnEvent, DuplicationEvent, GameObject, ParentChangeEvent,
    SpawnEvent as SpawnRequestEvent,
};
use crate::world_serialization::{WorldLoadRequest, WorldSaveRequest};
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
    level_name: String,
    save_name: String,
    parent_name: String,
    entity_name: String,
    spawn_item: GameObject,
}

impl Default for SceneEditorState {
    fn default() -> Self {
        Self {
            level_name: "demo".to_owned(),
            save_name: default(),
            active: default(),
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

#[allow(clippy::too_many_arguments)]
fn show_editor(
    mut egui_context: ResMut<EguiContext>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut level_save_events: EventWriter<WorldSaveRequest>,
    mut level_load_events: EventWriter<WorldLoadRequest>,
    mut game_save_events: EventWriter<GameSaveRequest>,
    mut game_load_events: EventWriter<GameLoadRequest>,
    mut parenting_events: EventWriter<ParentChangeEvent>,
    mut duplication_events: EventWriter<DuplicationEvent>,
    mut state: ResMut<SceneEditorState>,
    mut delayed_spawner: EventWriter<DelayedSpawnEvent>,
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
                ui.label("Level name: ");
                ui.text_edit_singleline(&mut state.level_name);
            });

            ui.add_enabled_ui(!state.level_name.is_empty(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        level_save_events.send(WorldSaveRequest {
                            filename: state.level_name.clone(),
                        })
                    }
                    if ui.button("Load").clicked() {
                        level_load_events.send(WorldLoadRequest {
                            filename: state.level_name.clone(),
                        });
                        // Make sure the player is spawned after the level
                        delayed_spawner.send(DelayedSpawnEvent {
                            tick_delay: 2,
                            event: SpawnRequestEvent {
                                object: GameObject::Player,
                                transform: Transform::from_translation((0., 0.5, 0.).into()),
                                parent: None,
                                name: Some("Player".into()),
                            },
                        });
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
                    game_save_events.send(GameSaveRequest {
                        filename: filename.clone(),
                    })
                }
                if ui.button("Load").clicked() {
                    game_load_events.send(GameLoadRequest { filename });
                }
            });

            ui.separator();
            ui.heading("Entity Control");
            ui.horizontal(|ui| {
                ui.label("Entity:");
                ui.text_edit_singleline(&mut state.entity_name);
            });
            let has_entity = !state.entity_name.is_empty();

            ui.add_space(10.);
            ui.horizontal(|ui| {
                ui.label("New Parent:");
                ui.text_edit_singleline(&mut state.parent_name);
            });
            let has_valid_parent = !state.parent_name.is_empty()
                && has_entity
                && state.entity_name != state.parent_name;
            ui.horizontal(|ui| {
                ui.add_enabled_ui(has_valid_parent, |ui| {
                    if ui.button("Set Parent").clicked() {
                        parenting_events.send(ParentChangeEvent {
                            name: state.entity_name.clone().into(),
                            new_parent: Some(state.parent_name.clone().into()),
                        });
                    }
                });
                if ui.button("Remove Parent").clicked() {
                    parenting_events.send(ParentChangeEvent {
                        name: state.entity_name.clone().into(),
                        new_parent: None,
                    });
                }
            });

            ui.add_space(10.);
            ui.label("Spawning");
            ui.horizontal(|ui| {
                ui.add_enabled_ui(has_entity, |ui| {
                    if ui.button("Duplicate").clicked() {
                        duplication_events.send(DuplicationEvent {
                            name: state.entity_name.clone().into(),
                        });
                    }
                });
                if ui.button("Spawn").clicked() {
                    let name = state.entity_name.clone();
                    let name = (!name.is_empty()).then(|| name.into());
                    let parent =
                        (!state.parent_name.is_empty()).then(|| state.parent_name.clone().into());

                    spawn_events.send(SpawnEvent {
                        object: state.spawn_item,
                        name,
                        parent,
                    });
                }
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
