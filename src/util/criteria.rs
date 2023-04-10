use crate::player_control::actions::ActionsFrozen;
use bevy::prelude::*;

pub(crate) fn is_frozen(actions_frozen: Res<ActionsFrozen>) -> bool {
    actions_frozen.is_frozen()
}

#[allow(unused)]
pub(crate) fn never() -> bool {
    false
}
