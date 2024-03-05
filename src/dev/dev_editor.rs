use crate::{player_control::camera::ForceCursorGrabMode, GameState};
use anyhow::Context;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_editor_pls::{
    editor::{Editor, EditorEvent},
    editor_window::EditorWindow,
    AddEditorWindow,
};
use bevy_egui::egui;
use bevy_mod_sysfail::prelude::*;
use bevy_xpbd_3d::prelude::PhysicsGizmos;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
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
        _world: &mut World,
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
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
#[derive(Default)]
pub(crate) struct DevEditorState {
    pub(crate) open: bool,
    pub(crate) collider_render_enabled: bool,
    pub(crate) navmesh_render_enabled: bool,
}

#[sysfail(Log<anyhow::Error, Error>)]
fn handle_debug_render(
    state: Res<Editor>,
    mut last_enabled: Local<bool>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let current_enabled = state
        .window_state::<DevEditorWindow>()
        .context("Failed to read dev window state")?
        .collider_render_enabled;
    if current_enabled == *last_enabled {
        return Ok(());
    }
    *last_enabled = current_enabled;
    let config = config_store.config_mut::<PhysicsGizmos>().0;
    config.enabled = current_enabled;
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
