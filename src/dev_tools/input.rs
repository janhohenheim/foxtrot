//! Input for the dev tools.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_dev_tools_input);
    app.add_input_context::<DevToolsInputContext>();
    app.add_observer(dev_tools_input_binding);
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub(crate) struct ToggleDebugUi;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub(crate) struct ForceFreeCursor;

#[derive(Debug, InputContext, Default)]
struct DevToolsInputContext;

fn dev_tools_input_binding(
    _trigger: Trigger<Binding<DevToolsInputContext>>,
    mut actions: Single<&mut Actions<DevToolsInputContext>>,
) {
    actions.bind::<ToggleDebugUi>().to(KeyCode::F3);
    actions.bind::<ForceFreeCursor>().to(KeyCode::Backquote);
}

fn setup_dev_tools_input(mut commands: Commands) {
    commands.spawn((
        Name::new("DevToolsInput"),
        Actions::<DevToolsInputContext>::default(),
    ));
}
