use crate::level_instanciation::spawning::event::ParentChangeEvent;
use crate::level_instanciation::spawning::spawn_container::SpawnContainerRegistry;
use bevy::prelude::*;

pub fn change_parent(
    mut commands: Commands,
    mut parent_changes: EventReader<ParentChangeEvent>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    for change in parent_changes.iter() {
        let child = match spawn_containers.0.get(&change.name) {
            None => {
                error!("Failed to fetch child: {}", change.name);
                continue;
            }
            Some(&entity) => entity,
        };

        if let Some(parent) = change.new_parent.clone() {
            let parent = spawn_containers.get_or_spawn(parent, &mut commands);
            commands
                .get_entity(child)
                .unwrap_or_else(|| panic!("Failed to fetch entity with name {}", change.name))
                .set_parent(parent);
        } else {
            commands
                .get_entity(child)
                .unwrap_or_else(|| panic!("Failed to fetch entity with name {}", change.name))
                .remove_parent();
        }
    }
}
