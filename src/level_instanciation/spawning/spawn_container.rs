use crate::level_instanciation::spawning::event::{
    DuplicationEvent, ParentChangeEvent, SpawnEvent,
};
use crate::level_instanciation::spawning::SpawnTracker;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct SpawnContainerRegistry(pub HashMap<Cow<'static, str>, Entity>);

impl SpawnContainerRegistry {
    pub fn get_or_spawn(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        commands: &mut Commands,
    ) -> Entity {
        // command.spawn() takes a tick to actually spawn stuff,
        // so we need to keep an own list of already "spawned" parents
        let name = name.into();
        let spawn_parent = |commands: &mut Commands| {
            commands
                .spawn((
                    Name::new(name.clone()),
                    VisibilityBundle::default(),
                    TransformBundle::default(),
                    SpawnTracker::default(),
                ))
                .id()
        };
        let parent = *self
            .0
            .entry(name.clone())
            .or_insert_with(|| spawn_parent(commands));

        if commands.get_entity(parent).is_some() {
            parent
        } else {
            // parent was removed at some prior point
            let entity = spawn_parent(commands);
            self.0.insert(name.clone(), entity);
            entity
        }
    }
}

pub fn sync_container_registry(
    name_query: Query<(Entity, &Name), With<SpawnTracker>>,
    spawn_events: EventReader<SpawnEvent>,
    parenting_events: EventReader<ParentChangeEvent>,
    duplication_events: EventReader<DuplicationEvent>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    if spawn_events.is_empty() && parenting_events.is_empty() && duplication_events.is_empty() {
        return;
    }
    spawn_containers.0 = default();

    for (entity, name) in name_query.iter() {
        let name = name.to_string();
        spawn_containers.0.insert(name.clone().into(), entity);
    }
}
