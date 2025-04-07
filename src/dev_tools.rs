//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_plugins((DebugUiPlugin, PhysicsDebugPlugin::default()));
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
    app.add_systems(
        Update,
        (toggle_debug_ui, toggle_physics_debug_ui).run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_physics_debug_ui(mut config_store: ResMut<GizmoConfigStore>) {
    let config = config_store.config_mut::<PhysicsGizmos>().0;
    config.enabled = !config.enabled;
}
