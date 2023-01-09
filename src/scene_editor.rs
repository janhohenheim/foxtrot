use crate::actions::{Actions, ActionsFrozen};
use crate::camera::{get_raycast_location, PlayerCamera};
use crate::game_objects::{GameObjects, Object};
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::{Align, ScrollArea};
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub struct SceneEditorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SceneEditorStatus {
    active: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SpawnEvent(pub Object);

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        app.add_event::<SpawnEvent>()
            .init_resource::<SceneEditorStatus>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_toggle)
                    .with_system(show_editor)
                    .with_system(spawn_objects),
            );

        let _ = app;
    }
}

const MAX_SPAWN_DISTANCE: f32 = 30.0;

fn handle_toggle(
    mut commands: Commands,
    actions: Res<Actions>,
    mut scene_editor_status: ResMut<SceneEditorStatus>,
) {
    if !actions.toggle_editor {
        return;
    }
    scene_editor_status.active = !scene_editor_status.active;

    if scene_editor_status.active {
        commands.init_resource::<ActionsFrozen>();
    } else {
        commands.remove_resource::<ActionsFrozen>();
    }
}

fn show_editor(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    scene_editor_status: Res<SceneEditorStatus>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    if !scene_editor_status.active {
        return;
    }
    let window = windows.get_primary().unwrap();
    egui::Window::new("Scene Editor")
        .collapsible(false)
        .title_bar(false)
        .auto_sized()
        .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .max_height(200.0)
                .auto_shrink([true; 2])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for item in Object::iter() {
                            let item_to_track = Object::Grass;
                            let track_item = false;
                            let item_to_track_align = Some(Align::Center);
                            let response = ui.button(format!("{item:?}"));
                            if track_item && item == item_to_track {
                                response.scroll_to_me(item_to_track_align)
                            }
                            if response.clicked() {
                                spawn_events.send(SpawnEvent(item));
                            }
                        }
                    });
                });
        });
}

fn spawn_objects(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent>,
    game_objects: Res<GameObjects>,
    asset_server: Res<AssetServer>,
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

    let game_objects = game_objects.retrieve_with(asset_server);

    for object in spawn_events.iter() {
        let location = get_raycast_location(camera, player, &rapier_context, MAX_SPAWN_DISTANCE);
        let bundle = game_objects.get(&object.0, Transform::from_translation(location));
        commands.spawn(bundle);
    }
}
