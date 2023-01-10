use crate::actions::{Actions, ActionsFrozen};
use crate::camera::{get_raycast_location, PlayerCamera};
use crate::player::Player;
use crate::spawning::{GameObject, SpawnEvent as SpawnRequestEvent};
use crate::world_serialization::{LoadRequest, SaveRequest};
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::{Align, ScrollArea};
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub struct SceneEditorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SceneEditorState {
    active: bool,
    save_name: String,
}

impl Default for SceneEditorState {
    fn default() -> Self {
        Self {
            save_name: "demo".to_owned(),
            active: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SpawnEvent(pub GameObject);

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

const MAX_SPAWN_DISTANCE: f32 = 30.0;

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
                egui::TextEdit::singleline(&mut editor_state.save_name).show(ui);
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
            ui.label("Spawn object");
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
                                    spawn_events.send(SpawnEvent(item));
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
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    rapier_context: Res<RapierContext>,
) {
    let camera = match camera_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    let player = match player_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };

    for object in spawn_requests.iter() {
        let eye_height_offset = Vec3::new(0., 1., 0.);
        let origin = player.translation + eye_height_offset;
        let direction = -(camera.rotation * Vec3::Z);

        let offset_to_not_spawn_in_ground = Vec3::new(0., 2., 0.);
        let location =
            get_raycast_location(&origin, &direction, &rapier_context, MAX_SPAWN_DISTANCE)
                + offset_to_not_spawn_in_ground;

        spawn_requester.send(SpawnRequestEvent {
            object: object.0,
            transform: Transform::from_translation(location),
            parent: None,
        });
    }
}
