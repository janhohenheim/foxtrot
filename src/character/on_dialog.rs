use bevy::prelude::*;
use bevy_tnua::prelude::TnuaController;

use crate::dialog::StartDialog;

pub(super) fn plugin(app: &mut App) {
    app.observe(turn_towards_player);
}

fn turn_towards_player(
    trigger: Trigger<StartDialog>,
    q_character: Query<(), With<TnuaController>>,
) {
    let Ok(_) = q_character.get(trigger.entity()) else {
        return;
    };
    // Todo: insert a component that makes the character turn towards the player in fixed timesteps.
}
