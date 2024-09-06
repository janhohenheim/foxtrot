//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::screens::{gameplay::GameplayState, Screen};
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
    app.add_systems(
        Update,
        (log_transitions::<Screen>, log_transitions::<GameplayState>),
    );

    app.add_plugins((
        DebugUiPlugin,
        WorldInspectorPlugin::default().run_if(is_inspector_active),
        PhysicsDebugPlugin::default(),
    ));
    app.add_systems(
        Update,
        toggle_debug_mode.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.init_resource::<InspectorActive>();

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

#[derive(Resource, Default)]
struct InspectorActive(bool);

fn toggle_debug_mode(
    mut options: ResMut<UiDebugOptions>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut is_inspector_active: ResMut<InspectorActive>,
) {
    options.toggle();
    let physics_gizmos = config_store.config_mut::<PhysicsGizmos>().0;
    physics_gizmos.enabled = options.enabled;
    is_inspector_active.0 = options.enabled;
}

fn is_inspector_active(is_active: Res<InspectorActive>) -> bool {
    is_active.as_ref().0
}
