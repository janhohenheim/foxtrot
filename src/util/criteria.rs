use crate::player_control::actions::ActionsFrozen;
use bevy::prelude::*;

pub fn is_frozen(actions_frozen: Res<ActionsFrozen>) -> bool {
    actions_frozen.is_frozen()
}

#[allow(unused)]
pub fn never() -> bool {
    false
}
