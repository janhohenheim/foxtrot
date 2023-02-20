use crate::dev::dev_editor::DevEditorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;

pub mod dev_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_plugin(EditorPlugin)
                .insert_resource(default_editor_controls())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(DebugLinesPlugin::default())
                .add_plugin(DevEditorPlugin)
                .add_plugin(LogDiagnosticsPlugin::filtered(vec![]))
                .add_plugin(RapierDebugRenderPlugin {
                    enabled: false,
                    ..default()
                });
        }
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
