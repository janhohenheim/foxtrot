use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Despawn {
    pub recursive: bool,
}

pub fn despawn(mut commands: Commands, despawn_query: Query<(Entity, &Despawn, &Children)>) {
    for (entity, despawn, children) in despawn_query.iter() {
        if despawn.recursive {
            commands.entity(entity).despawn_recursive();
        } else {
            for child in children.iter() {
                commands.entity(*child).remove_parent();
            }
        }
    }
}
