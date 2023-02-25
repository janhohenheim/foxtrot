use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::event::SpawnEvent;
use crate::level_instantiation::spawning::{DelayedSpawnEvent, GameObjectSpawner, SpawnTracker};
use crate::shader::Materials;
use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub fn spawn_requested(
    mut commands: Commands,
    scenes: Res<Assets<Scene>>,
    materials: Res<Materials>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
    animations: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) -> Result<()> {
    for spawn in spawn_requests.iter() {
        let entity = spawner
            .attach(
                &mut commands,
                &scenes,
                &materials,
                &animations,
                &scene_handles,
            )
            .spawn(spawn.object, spawn.transform)?;
        commands
            .entity(entity)
            .insert(SpawnTracker::from(spawn.clone()));
    }
    Ok(())
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
