use crate::file_system_interaction::game_state_serialization::{GameLoadRequest, GameSaveRequest};
use crate::file_system_interaction::level_serialization::{WorldLoadRequest, WorldSaveRequest};
use crate::level_instantiation::spawning::{DelayedSpawnEvent, GameObject, SpawnEvent};
use crate::GameState;
use bevy::prelude::*;
use bevy_editor_pls::editor_window::EditorWindow;
use bevy_editor_pls::{AddEditorWindow, Editor};
use bevy_egui::egui;
use bevy_egui::egui::ScrollArea;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::NavMesh;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub struct SceneEditorPlugin;

pub struct FoxtrotDevWindow;

impl EditorWindow for FoxtrotDevWindow {
    const DEFAULT_SIZE: (f32, f32) = (200., 150.);
    const NAME: &'static str = "Foxtrot Dev";
    type State = SceneEditorState;
    fn ui(
        world: &mut World,
        mut cx: bevy_editor_pls::editor_window::EditorWindowContext,
        ui: &mut egui::Ui,
    ) {
        let state = cx
            .state_mut::<FoxtrotDevWindow>()
            .expect("Window State Loaded");
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
                    world.send_event(DelayedSpawnEvent {
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
            world.send_event(SpawnEvent {
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
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SceneEditorState {
    pub level_name: String,
    pub save_name: String,
    pub spawn_item: GameObject,
    pub collider_render_enabled: bool,
    pub navmesh_render_enabled: bool,
}

impl Default for SceneEditorState {
    fn default() -> Self {
        Self {
            level_name: "old_town".to_owned(),
            save_name: default(),
            spawn_item: default(),
            collider_render_enabled: false,
            navmesh_render_enabled: false,
        }
    }
}

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneEditorState>()
            .add_editor_window::<FoxtrotDevWindow>()
            .insert_resource(default_editor_controls())
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_debug_render)
                    .with_system(handle_navmesh_render),
            );
    }
}

fn handle_debug_render(state: Res<Editor>, mut debug_render_context: ResMut<DebugRenderContext>) {
    debug_render_context.enabled = state
        .window_state::<FoxtrotDevWindow>()
        .expect("Window State Loaded")
        .collider_render_enabled;
}

fn handle_navmesh_render(
    state: Res<Editor>,
    nav_mesh: Res<NavMesh>,
    mut lines: ResMut<DebugLines>,
) {
    if !state
        .window_state::<FoxtrotDevWindow>()
        .expect("Window State Loaded")
        .navmesh_render_enabled
    {
        return;
    }

    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (tile_coord, tile) in nav_mesh.get_tiles().iter() {
            let tile_color = Color::Rgba {
                red: 0.0,
                green: (tile_coord.x % 10) as f32 / 10.0,
                blue: (tile_coord.y % 10) as f32 / 10.0,
                alpha: 1.0,
            };
            // Draw polygons.
            for poly in tile.polygons.iter() {
                let indices = &poly.indices;
                for i in 0..indices.len() {
                    let a = tile.vertices[indices[i] as usize];
                    let b = tile.vertices[indices[(i + 1) % indices.len()] as usize];

                    lines.line_colored(a, b, 0.0, tile_color);
                }
            }

            // Draw vertex points.
            for vertex in tile.vertices.iter() {
                lines.line_colored(*vertex, *vertex + Vec3::Y, 0.0, tile_color);
            }
        }
    }
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut start = EditorControls::default_bindings();
    start.unbind(Action::PlayPauseEditor);
    start.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::Q)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    start
}
