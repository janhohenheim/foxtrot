use crate::spawning::{SpawnEvent, SpawnTracker};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, iter};

pub struct WorldSerializationPlugin;

impl Plugin for WorldSerializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveRequest>()
            .add_event::<LoadRequest>()
            .add_system(save_world)
            .add_system(load_world);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct SaveRequest {
    pub filename: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct LoadRequest {
    pub filename: String,
}

fn save_world(
    mut save_requests: EventReader<SaveRequest>,
    spawn_query: Query<(&SpawnTracker, &Transform)>,
) {
    for save in save_requests.iter() {
        let scene = save.filename.clone();
        let valid_candidates: Vec<_> = iter::once(scene.clone())
            .chain((1..).into_iter().map(|n| format!("{0}-{n}", scene.clone())))
            .map(|filename| {
                Path::new("assets")
                    .join("scenes")
                    .join(format!("{filename}.scn.ron"))
            })
            .map(|path| (path.clone(), fs::try_exists(path).ok()))
            .take(10)
            .filter_map(|(path, maybe_exists)| maybe_exists.map(|exists| (path, exists)))
            .collect();
        if valid_candidates.is_empty() {
            error!("Failed to save scene \"{}\": Invalid path", scene);
        } else {
            if let Some(path) = valid_candidates
                .iter()
                .filter_map(|(path, exists)| (!exists).then(|| path))
                .next()
            {
                let serialized_world = serialize_world(&spawn_query);
                fs::write(path, serialized_world)
                    .unwrap_or_else(|e| error!("Failed to save scene \"{}\": {}", scene, e));
                info!(
                    "Successfully saved scene \"{}\" at {}",
                    scene,
                    path.to_string_lossy()
                );
            } else {
                error!(
                    "Failed to save scene \"{}\": Already got too many saves with this name",
                    scene
                );
            }
        }
    }
}

fn load_world(
    mut commands: Commands,
    mut load_requests: EventReader<LoadRequest>,
    current_spawn_query: Query<Entity, (With<SpawnTracker>, With<Transform>)>,
    mut spawn_requests: EventWriter<SpawnEvent>,
) {
    for load in load_requests.iter() {
        let path = Path::new("assets")
            .join("scenes")
            .join(format!("{}.scn.ron", load.filename));
        match fs::read_to_string(&path) {
            Ok(serialized_world) => {
                let spawn_events = deserialize_world(&serialized_world);
                for entity in &current_spawn_query {
                    commands.entity(entity).despawn_recursive();
                }
                for event in spawn_events {
                    spawn_requests.send(event);
                }
                info!(
                    "Successfully loaded scene \"{}\" from {}",
                    load.filename,
                    path.to_string_lossy()
                )
            }
            Err(e) => error!("Failed to load scene \"{}\": {}", load.filename, e),
        };
    }
}

fn serialize_world(spawn_query: &Query<(&SpawnTracker, &Transform)>) -> String {
    let objects: Vec<_> = spawn_query
        .iter()
        .map(|(spawn_tracker, transform)| SpawnEvent {
            object: spawn_tracker.object,
            transform: *transform,
            name: spawn_tracker.name.clone(),
            parent: spawn_tracker.parent.clone(),
        })
        .collect();
    ron::to_string(&objects).expect("Failed to serialize world")
}

fn deserialize_world(serialized_world: &str) -> Vec<SpawnEvent> {
    ron::from_str(serialized_world).expect("Failed to deserialize world")
}
