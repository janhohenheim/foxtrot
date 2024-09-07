use bevy::{app::RunFixedMainLoop, prelude::*, time::run_fixed_main_schedule};
use bevy_tnua::TnuaSystemSet;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        RunFixedMainLoop,
        (
            VariableBeforeFixedGameSet::CharacterController,
            TnuaSystemSet.before(run_fixed_main_schedule),
        )
            .chain(),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum VariableBeforeFixedGameSet {
    CharacterController,
}
