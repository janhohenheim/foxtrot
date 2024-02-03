use crate::dev::dev_editor::dev_editor_plugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_xpbd_3d::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub(crate) mod dev_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub(crate) fn dev_plugin(app: &mut App) {
    {
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .fn_plugin(dev_editor_plugin)
            .add_plugins(LogDiagnosticsPlugin::filtered(vec![]))
            .add_plugins(PhysicsDebugPlugin::default())
            .insert_resource(PhysicsDebugConfig {
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
            input: UserInput::Single(Button::Keyboard(KeyCode::G)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}
