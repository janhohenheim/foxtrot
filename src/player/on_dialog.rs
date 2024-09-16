use bevy::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::{character::controller::ControllerDisabled, dialog::StartDialog};

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.observe(disable_controller_on_dialog);
    // Todo: schedule this
    app.add_systems(Update, enable_controller_on_dialog_end);
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

fn enable_controller_on_dialog_end(
    mut dialog_complete_reader: EventReader<DialogueCompleteEvent>,
    q_player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    for _ in dialog_complete_reader.read() {
        for player in &q_player {
            commands.entity(player).remove::<ControllerDisabled>();
        }
    }
}
