use bevy::prelude::*;

use crate::{character::controller::ControllerDisabled, dialog::StartDialog};

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.observe(disable_controller_on_dialog);
}

fn disable_controller_on_dialog(
    _trigger: Trigger<StartDialog>,
    q_player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    for player in &q_player {
        commands.entity(player).insert(ControllerDisabled);
    }
}
