use bevy::prelude::*;

#[cfg(feature = "dev")]
use bevy_editor_pls::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

/// Plugin with debugging utility intendend for use during development only.
/// Will not do anything when used in a release build.
pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(EditorPlugin);
        }
        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
