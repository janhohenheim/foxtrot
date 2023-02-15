use crate::file_system_interaction::game_state_serialization::{GameLoadRequest, GameSaveRequest};
use crate::file_system_interaction::level_serialization::{WorldLoadRequest, WorldSaveRequest};
use crate::level_instantiation::spawning::{DelayedSpawnEvent, GameObject, SpawnEvent};
use crate::movement::navigation::navmesh::NavMesh;
use crate::player_control::actions::{Actions, ActionsFrozen};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::ScrollArea;
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub struct SceneEditorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SceneEditorState {
    active: bool,
    level_name: String,
    save_name: String,
    spawn_item: GameObject,
    collider_render_enabled: bool,
    navmesh_render_enabled: bool,
}

impl Default for SceneEditorState {
    fn default() -> Self {
        Self {
            level_name: "old_town".to_owned(),
            save_name: default(),
            active: default(),
            spawn_item: default(),
            collider_render_enabled: default(),
            navmesh_render_enabled: true,
        }
    }
}

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneEditorState>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_toggle)
                .with_system(show_editor)
                .with_system(handle_debug_render)
                .with_system(handle_navmesh_render),
        );
    }
}

fn handle_toggle(
    actions: Res<Actions>,
    mut scene_editor_state: ResMut<SceneEditorState>,
    mut actions_frozen: ResMut<ActionsFrozen>,
) {
    if !actions.ui.toggle_editor {
        return;
    }
    scene_editor_state.active = !scene_editor_state.active;

    if scene_editor_state.active {
        actions_frozen.freeze();
    } else {
        actions_frozen.unfreeze();
    }
}

fn handle_debug_render(
    state: Res<SceneEditorState>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    debug_render_context.enabled = state.collider_render_enabled;
}

fn handle_navmesh_render(
    state: Res<SceneEditorState>,
    mut with_navmesh: Query<&mut Visibility, With<NavMesh>>,
) {
    for mut visibility in with_navmesh.iter_mut() {
        visibility.is_visible = state.navmesh_render_enabled;
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
                            event: SpawnEvent {
                                object: GameObject::Player,
                                transform: Transform::from_translation((0., 1.5, 0.).into()),
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

            ui.add_space(10.);
            ui.label("Spawning");
            if ui.button("Spawn").clicked() {
                spawn_events.send(SpawnEvent {
                    object: state.spawn_item,
                    ..default()
                });
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
        });
}
