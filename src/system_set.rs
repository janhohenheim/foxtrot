use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(FixedUpdate, (FixedGameSet::CharacterController,).chain());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum FixedGameSet {
    CharacterController,
}
