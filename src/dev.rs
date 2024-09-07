use avian3d::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_editor_pls::prelude::*;

pub(crate) mod dev_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub(super) fn plugin(app: &mut App) {
    {
        app.add_plugins((
            WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::filtered(vec![]),
            PhysicsDebugPlugin::default(),
        ))

        // .insert_gizmo_group(
        //     PhysicsGizmos {
        //         aabb_color: Some(Color::WHITE),
        //         ..default()
        //     },
        //     GizmoConfig {
        //         enabled: false,
        //         ..default()
        //     },
        // )
        ;
    }
}
