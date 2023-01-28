use crate::level_instanciation::spawning::counter::Counter;
use crate::level_instanciation::spawning::event::{DuplicationEvent, SpawnEvent};
use crate::level_instanciation::spawning::spawn_container::SpawnContainerRegistry;
use crate::level_instanciation::spawning::SpawnTracker;
use bevy::prelude::*;
use regex::Regex;
use std::borrow::Cow;

pub fn duplicate(
    mut duplication_requests: EventReader<DuplicationEvent>,
    spawn_containers: Res<SpawnContainerRegistry>,
    mut spawn_requests: EventWriter<SpawnEvent>,
    spawn_tracker_query: Query<(&SpawnTracker, &Name, Option<&Transform>, &Children)>,
    parent_query: Query<&Parent>,
    mut counter: ResMut<Counter>,
) {
    for duplication in duplication_requests.iter() {
        let entity = match spawn_containers.0.get(&duplication.name) {
            None => {
                error!(
                    "Failed to find entity \"{}\" for duplication",
                    duplication.name
                );
                continue;
            }
            Some(entity) => *entity,
        };
        let parent = parent_query
            .get(entity)
            .ok()
            .and_then(|parent| spawn_tracker_query.get(parent.get()).ok())
            .map(|(_, name, _, _)| name.to_string().into());
        send_recursive_spawn_events(
            entity,
            parent,
            &mut spawn_requests,
            &spawn_tracker_query,
            &mut counter,
        );
    }
}

fn send_recursive_spawn_events(
    entity: Entity,
    parent: Option<Cow<'static, str>>,
    spawn_requests: &mut EventWriter<SpawnEvent>,
    spawn_tracker_query: &Query<(&SpawnTracker, &Name, Option<&Transform>, &Children)>,
    counter: &mut ResMut<Counter>,
) {
    let (spawn_tracker, name, transform, children) = match spawn_tracker_query.get(entity) {
        Ok(result) => result,
        Err(_) => {
            return;
        }
    };

    let name = name.to_string();

    let re = Regex::new(r"^(.*) \d+$").unwrap();
    let name = if let Some(captures) = re.captures(&name) && let Some(unnumbered_name) = captures.get(1) {
        unnumbered_name.as_str()
    } else {
        &name
    };
    let number = counter.next(name);
    let name = Some(format!("{name} {number}").into());

    spawn_requests.send(SpawnEvent {
        object: spawn_tracker.object,
        transform: transform.map(Clone::clone).unwrap_or_default(),
        parent,
        name: name.clone(),
    });
    for &child in children {
        send_recursive_spawn_events(
            child,
            name.clone(),
            spawn_requests,
            spawn_tracker_query,
            counter,
        );
    }
}
