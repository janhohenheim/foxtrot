use crate::actions::{Actions, ActionsFrozen};
use crate::camera::{get_raycast_location, PlayerCamera};
use crate::game_objects::{GameObjectSpawner, GameObject};
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
struct SpawnEvent(pub GameObject);

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
                        for item in GameObject::iter() {
                            let item_to_track = GameObject::Grass;
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
    spawner: Res<GameObjectSpawner>,
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

    for object in spawn_events.iter() {
        let eye_height_offset = Vec3::new(0., 1., 0.);
        let origin = player.translation + eye_height_offset;
        let direction = -(camera.rotation * Vec3::Z);

        let offset_to_not_spawn_in_ground = Vec3::new(0., 2., 0.);
        let location =
            get_raycast_location(&origin, &direction, &rapier_context, MAX_SPAWN_DISTANCE)
                + offset_to_not_spawn_in_ground;

        spawner
            .attach(&asset_server, &mut commands)
            .spawn(&object.0, Transform::from_translation(location));
    }
}
