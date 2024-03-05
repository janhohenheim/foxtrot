use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::PhysicsSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
struct Hidden;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Hidden>().add_systems(
        Update,
        spawn
            .after(PhysicsSet::Sync)
            .run_if(in_state(GameState::Playing)),
    );
}

fn spawn(hidden: Query<Entity, Added<Hidden>>, mut commands: Commands) {
    for entity in hidden.iter() {
        commands
            .entity(entity)
            .insert(Visibility::Hidden)
            .remove::<Hidden>();
    }
}
