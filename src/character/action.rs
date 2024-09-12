use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((InputManagerPlugin::<CharacterAction>::default(),));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Actionlike, Reflect, Default)]
pub(crate) enum CharacterAction {
    #[default]
    #[actionlike(DualAxis)]
    Move,
    Sprint,
    Jump,
    Interact,
}
