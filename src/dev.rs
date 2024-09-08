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
    
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls())
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                dev_editor::plugin,
                LogDiagnosticsPlugin::filtered(vec![]),
                PhysicsDebugPlugin::default(),
            ))
            .init_gizmo_group::<DefaultGizmoConfigGroup>();

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