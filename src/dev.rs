use bevy::prelude::*;
#[cfg(feature = "editor")]
use bevy_prototype_debug_lines::DebugLinesPlugin;

#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;

#[cfg(feature = "editor")]
use crate::scene_editor::SceneEditorPlugin;
#[cfg(feature = "editor")]
use bevy::diagnostic::LogDiagnosticsPlugin;

/// Plugin with debugging utility intended for use during development only.
/// Will not do anything when used in a release build.
pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        {
            app.add_plugin(EditorPlugin)
                .add_plugin(DebugLinesPlugin::default())
                .add_plugin(SceneEditorPlugin)
                .add_plugin(LogDiagnosticsPlugin::default());
        }

        #[cfg(not(feature = "editor"))]
        {
            // Suppress warning of unused app in release builds.
            let _ = app;
        }
    }
}
