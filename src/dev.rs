use crate::dev::scene_editor::SceneEditorPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;

mod scene_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_plugin(EditorPlugin)
                .add_plugin(DebugLinesPlugin::default())
                .add_plugin(SceneEditorPlugin)
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin {
                    enabled: false,
                    ..default()
                });
        }
    }
}
