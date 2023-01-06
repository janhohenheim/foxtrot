use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::actions::{Actions, ActionsFrozen};
use crate::GameState;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub struct SceneEditorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SceneEditorStatus {
    active: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColliderCreationData {}

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        app.init_resource::<SceneEditorStatus>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_toggle)
                .with_system(show_editor),
        );

        let _ = app;
    }
}

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
    mut scene_editor_status: ResMut<SceneEditorStatus>,
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
        .show(
            egui_context.ctx_mut(),
            |ui| {
                if ui.button("foo").clicked() {}
            },
        );
}
