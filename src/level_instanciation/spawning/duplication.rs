use crate::level_instanciation::spawning::event::{DuplicationEvent, SpawnEvent};
use crate::level_instanciation::spawning::SpawnTracker;
use bevy::prelude::*;

pub fn duplicate(
    mut duplication_requests: EventReader<DuplicationEvent>,
    mut spawn_requests: EventWriter<SpawnEvent>,
    spawn_tracker_query: Query<(&SpawnTracker, Option<&Transform>, &Children)>,
    name_query: Query<(Entity, &Name)>,
) {
    for duplication in duplication_requests.iter() {
        let entity = match name_query
            .iter()
            .find(|(_, name)| name.as_str() == duplication.name)
        {
            Some((entity, _)) => entity,
            None => {
                error!(
                    "Duplication failed: no entity found with name {}",
                    duplication.name
                );
                continue;
            }
        };
        send_recursive_spawn_events(entity, &mut spawn_requests, &spawn_tracker_query);
    }
}

fn send_recursive_spawn_events(
    entity: Entity,
    spawn_requests: &mut EventWriter<SpawnEvent>,
    spawn_tracker_query: &Query<(&SpawnTracker, Option<&Transform>, &Children)>,
) {
    let (spawn_tracker, transform, children) = match spawn_tracker_query.get(entity) {
        Ok(result) => result,
        Err(_) => {
            return;
        }
    };

    spawn_requests.send(SpawnEvent {
        object: spawn_tracker.object,
        transform: transform.map(Clone::clone).unwrap_or_default(),
    });
    for &child in children {
        send_recursive_spawn_events(child, spawn_requests, spawn_tracker_query);
    }
}
