use crate::spawning::counter::Counter;
use crate::spawning::event::SpawnEvent;
use crate::spawning::spawn_container::SpawnContainerRegistry;
use crate::spawning::{GameObject, GameObjectSpawner, SpawnTracker};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use regex::Regex;
use std::borrow::Cow;
use std::str::FromStr;

pub fn spawn_requested(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
    mut counter: ResMut<Counter>,
) {
    for spawn in spawn_requests.iter() {
        let name = spawn.get_name_or_default();

        let re = Regex::new(r"(^.*) (\d+)$").unwrap();
        if let Some(captures) = re.captures(&name)
            && let Some(name) = captures.get(1).map(|match_| match_.as_str().to_owned())
            && let Some(number) = captures.get(2)
            && let Ok(number) = usize::from_str(number.as_str()) {
            counter.set_at_least(&name, number);
        }

        let bundle = (
            Name::new(name.clone()),
            VisibilityBundle::default(),
            TransformBundle::from_transform(spawn.transform),
            SpawnTracker::from(spawn.clone()),
        );
        let spawn_children = |parent: &mut ChildBuilder| {
            spawner.attach(parent, &gltf).spawn(&spawn.object);
        };

        if let Some(ref parent_name) = spawn.parent {
            let parent_entity = spawn_containers.get_or_spawn(parent_name.clone(), &mut commands);
            if let Some(&existing_entity) = spawn_containers.0.get::<Cow<'static, str>>(&name.clone().into())
                && matches!(spawn.object, GameObject::Empty) {
                commands.get_entity(existing_entity).unwrap_or_else(|| panic!("Failed to fetch entity with name {}", name)).set_parent(parent_entity).insert(bundle);
            }  else {
                commands.get_entity(parent_entity).unwrap_or_else(|| panic!("Failed to fetch entity with name {}", parent_name)).with_children(|parent| {
                    let entity = parent.spawn(bundle).with_children(spawn_children).id();
                    spawn_containers.0.insert(name.into(), entity);
                });
            }
        } else {
            let entity = commands.spawn(bundle).with_children(spawn_children).id();
            spawn_containers.0.insert(name.into(), entity);
        }
    }
}
