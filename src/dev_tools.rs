//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::screens::Screen;
use avian3d::prelude::*;
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    app.add_plugins((
        DebugUiPlugin,
        WorldInspectorPlugin::new(),
        PhysicsDebugPlugin::default(),
    ));
    app.add_systems(
        Update,
        toggle_debug_mode.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    // Disable physics gizmos by default.
    app.insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::WHITE),
            ..default()
        },
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_mode(
    mut options: ResMut<UiDebugOptions>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    options.toggle();
    let physics_gizmos = config_store.config_mut::<PhysicsGizmos>().0;
    physics_gizmos.enabled = options.enabled;
}
