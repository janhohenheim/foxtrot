use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub(crate) mod dev_editor;

/// Plugin with debugging utility intended for use during development only.
/// Don't include this in a release build.
pub(super) fn plugin(app: &mut App) {
    {
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls())
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                dev_editor::plugin,
                LogDiagnosticsPlugin::filtered(vec![]),
                PhysicsDebugPlugin::default(),
            ))
            .insert_gizmo_group(
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
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::KeyQ)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}
