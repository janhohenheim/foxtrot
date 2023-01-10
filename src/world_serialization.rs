use crate::spawning::{SpawnEvent, SpawnTracker};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, iter};

pub struct WorldSerializationPlugin;

impl Plugin for WorldSerializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveRequest>().add_system(save_world);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct SaveRequest {
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
