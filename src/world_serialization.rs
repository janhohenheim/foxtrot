use crate::spawning::{SpawnEvent, SpawnTracker};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub struct WorldSerializationPlugin;

impl Plugin for WorldSerializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveRequest>().add_system(save_world);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct SaveRequest {
    filename: Option<String>,
}

fn save_world(
    time: Res<Time>,
    mut save_requests: EventReader<SaveRequest>,
    spawn_query: Query<(&SpawnTracker, &Transform)>,
) {
    for save in save_requests.iter() {
        let filename = save
            .filename
            .clone()
            .unwrap_or_else(|| format!("{}", time.elapsed_seconds()));
        let filename = format!("{filename}.ron");
        let path = Path::new("saves").join(filename);
        let serialized_world = serialize_world(&spawn_query);
        fs::write(path.clone(), serialized_world)
            .unwrap_or_else(|e| panic!("Failed to save {path:?}: {e}"));
    }
}

fn serialize_world(spawn_query: &Query<(&SpawnTracker, &Transform)>) -> String {
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
