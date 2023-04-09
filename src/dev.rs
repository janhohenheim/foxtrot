use crate::dev::dev_editor::dev_editor_plugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub(crate) mod dev_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub(crate) fn dev_plugin(app: &mut App) {
    {
        app.add_plugin(EditorPlugin)
            .insert_resource(default_editor_controls())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(DebugLinesPlugin::default())
            .fn_plugin(dev_editor_plugin)
            .add_plugin(LogDiagnosticsPlugin::filtered(vec![]))
            .add_plugin(RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            });
    }
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::Q)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}
