use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<DebugInputContext>();

    app.add_observer(debug_binding);
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub(crate) struct Noclip;

#[derive(Debug, InputContext, Default)]
pub(crate) struct DebugInputContext;

#[cfg_attr(feature = "hot_patch", hot)]
fn debug_binding(
    trigger: Trigger<Binding<DebugInputContext>>,
    mut players: Query<&mut Actions<DebugInputContext>>,
) {
    let mut actions = players.get_mut(trigger.target()).unwrap();

    actions.bind::<Noclip>().to(KeyCode::KeyN);
}
