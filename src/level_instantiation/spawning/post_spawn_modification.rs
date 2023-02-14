use crate::level_instantiation::spawning::spawn::Despawn;
use bevy::prelude::*;

pub fn set_hidden(mut added_name: Query<(&Name, &mut Visibility), Added<Name>>) {
    for (name, mut visibility) in added_name.iter_mut() {
        if name.to_lowercase().contains("[hidden]") {
            visibility.is_visible = false;
        }
    }
}

pub fn despawn_removed(
    mut commands: Commands,
    mut added_name: Query<(Entity, &Name), Added<Name>>,
) {
    for (entity, name) in added_name.iter_mut() {
        if name.to_lowercase().contains("[remove]") {
            commands.entity(entity).insert(Despawn { recursive: true });
        }
    }
}
