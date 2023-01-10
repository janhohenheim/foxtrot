use bevy::prelude::*;

use crate::spawning::{SpawnEvent, SpawnTracker};
use bevy_rapier3d::prelude::*;

pub struct WorldSerializationPlugin;

impl Plugin for WorldSerializationPlugin {
    fn build(&self, app: &mut App) {}
}

fn serialize_world(spawn_query: Query<(&SpawnTracker, &Transform)>) -> String {
    let objects: Vec<_> = spawn_query
        .iter()
        .map(|(spawn_tracker, transform)| SpawnEvent {
            object: spawn_tracker.object,
            transform: *transform,
            parent: spawn_tracker.parent.clone(),
        })
        .collect();
    ron::to_string(&objects).expect("Failed to serialize world")
}
