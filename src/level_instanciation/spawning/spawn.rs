use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instanciation::spawning::counter::Counter;
use crate::level_instanciation::spawning::event::SpawnEvent;
use crate::level_instanciation::spawning::spawn_container::SpawnContainerRegistry;
use crate::level_instanciation::spawning::{
    DelayedSpawnEvent, GameObject, GameObjectSpawner, SpawnEventParent, SpawnTracker,
};
use crate::shader::Materials;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

#[allow(clippy::too_many_arguments)]
pub fn spawn_requested(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    materials: Res<Materials>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
    mut counter: ResMut<Counter>,
    animations: Res<AnimationAssets>,
    scenes: Res<SceneAssets>,
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
            spawner
                .attach(parent, &gltf, &materials, &animations, &scenes)
                .spawn(spawn.object);
        };

        match spawn.parent {
            SpawnEventParent::Named(ref parent_name) => {
                let parent_entity =
                    spawn_containers.get_or_spawn(parent_name.clone(), &mut commands);
                if let Some(&existing_entity) = spawn_containers.0.get::<Cow<'static, str>>(&name.clone().into())
                    && matches!(spawn.object, GameObject::Empty) {
                    commands.get_entity(existing_entity).unwrap_or_else(|| panic!("Failed to fetch entity with name {name}")).set_parent(parent_entity).insert(bundle);
                }  else {
                    commands.get_entity(parent_entity).unwrap_or_else(|| panic!("Failed to fetch entity with name {parent_name}")).with_children(|parent| {
                        let entity = parent.spawn(bundle).with_children(spawn_children).id();
                        spawn_containers.0.insert(name.into(), entity);
                    });
                }
            }
            SpawnEventParent::Generated => {
                let entity = commands.spawn(bundle).with_children(spawn_children).id();
                spawn_containers.0.insert(name.into(), entity);
            }
            SpawnEventParent::None => {
                let entity = commands.spawn(bundle).with_children(spawn_children).id();
                commands.entity(entity).insert(Despawn { recursive: false });
            }
        }
    }
}

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
            //commands.entity(entity).despawn();
            for child in children.iter() {
                commands.entity(*child).remove_parent();
            }
        }
    }
}

#[derive(Debug, Resource, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct DelayedSpawnEvents(Vec<DelayedSpawnEvent>);

pub fn spawn_delayed(
    mut incoming_delayed_events: EventReader<DelayedSpawnEvent>,
    mut existing_delayed_events: ResMut<DelayedSpawnEvents>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    for delay in incoming_delayed_events.iter() {
        existing_delayed_events.0.push(delay.clone());
    }
    let mut events_to_delete = vec![];
    for (index, delay) in existing_delayed_events.0.iter_mut().enumerate() {
        let delay = delay.pass_tick();
        if delay.is_done() {
            spawn_events.send(delay.event.clone());
            events_to_delete.push(index)
        }
    }
    for index in events_to_delete.iter().rev() {
        existing_delayed_events.0.remove(*index);
    }
}
