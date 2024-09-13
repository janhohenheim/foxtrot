use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(FixedUpdate, (FixedGameSet::CharacterController,).chain());
    app.configure_sets(Update, (VariableGameSystem::Opportunities,).chain());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum FixedGameSet {
    CharacterController,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum VariableGameSystem {
    Opportunities,
}
