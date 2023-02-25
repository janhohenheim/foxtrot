use crate::file_system_interaction::asset_loading::LevelAssets;
use crate::level_instantiation::spawning::{
    GameObject, SpawnEvent, SpawnRequestedLabel, SpawnTracker,
};
use crate::util::log_error::log_errors;
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionOpportunities;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, iter};

pub struct LevelSerializationPlugin;

impl Plugin for LevelSerializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldSaveRequest>()
            .add_event::<WorldLoadRequest>()
            .add_system(save_world.pipe(log_errors).after(SpawnRequestedLabel))
            .add_system_to_stage(CoreStage::PostUpdate, load_world.pipe(log_errors));
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct WorldSaveRequest {
    pub filename: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct WorldLoadRequest {
    pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct CurrentLevel {
    pub scene: String,
}

fn save_world(
    mut save_requests: EventReader<WorldSaveRequest>,
    spawn_query: Query<(&SpawnTracker, Option<&Transform>)>,
) -> Result<()> {
    for save in save_requests.iter() {
        let scene = save.filename.clone();
        let valid_candidates: Vec<_> = iter::once(scene.clone())
            .chain((1..).map(|n| format!("{0}-{n}", scene.clone())))
            .map(|filename| {
                Path::new("assets")
                    .join("levels")
                    .join(format!("{filename}.lvl.ron"))
            })
            .map(|path| (path.clone(), fs::try_exists(path).ok()))
            .take(10)
            .filter_map(|(path, maybe_exists)| maybe_exists.map(|exists| (path, exists)))
            .collect();
        if valid_candidates.is_empty() {
            error!("Failed to save scene \"{}\": Invalid path", scene);
        } else if let Some(path) = valid_candidates
            .iter()
            .filter_map(|(path, exists)| (!exists).then_some(path))
            .next()
        {
            let serialized_world = serialize_world(&spawn_query)?;
            let dir = path.parent().context("Failed to get level directory")?;
            fs::create_dir_all(dir).context("Failed to create level directory")?;
            fs::write(path, serialized_world)
                .unwrap_or_else(|e| error!("Failed to save level \"{}\": {}", scene, e));
            info!(
                "Successfully saved level \"{}\" at {}",
                scene,
                path.to_string_lossy()
            );
        } else {
            error!(
                "Failed to save level \"{}\": Already got too many saves with this name",
                scene
            );
        }
    }
    Ok(())
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Protected;

fn load_world(
    mut commands: Commands,
    mut load_requests: EventReader<WorldLoadRequest>,
    current_spawn_query: Query<Entity, With<SpawnTracker>>,
    mut spawn_requests: EventWriter<SpawnEvent>,
    levels: Res<Assets<SerializedLevel>>,
    level_handles: Option<Res<LevelAssets>>,
) -> Result<()> {
    let level_handles = match level_handles {
        Some(level_handles) => level_handles,
        None => {
            return Ok(());
        }
    };
    for load in load_requests.iter() {
        let path = Path::new("levels")
            .join(format!("{}.lvl.ron", load.filename))
            .to_str()
            .with_context(|| {
                format!(
                    "Failed to convert path to string for filename: {}",
                    load.filename
                )
            })?
            .to_string();
        let handle = match level_handles.levels.get(&path) {
            Some(handle) => handle,
            None => {
                error!(
                    "Failed to load scene \"{}\": No such level. Available levels: {:?}",
                    path,
                    level_handles.levels.keys()
                );
                continue;
            }
        };
        let spawn_events = &levels
            .get(handle)
            .context("Failed to get level from handle in level assets")?
            .0;
        for entity in &current_spawn_query {
            commands
                .get_entity(entity)
                .context("Failed to get entity while loading")?
                .despawn_recursive();
        }
        for event in spawn_events {
            spawn_requests.send(event.clone());
        }
        commands.insert_resource(CurrentLevel {
            scene: load.filename.clone(),
        });
        commands.init_resource::<InteractionOpportunities>();
        commands.init_resource::<ActiveConditions>();
        commands.remove_resource::<CurrentDialog>();

        info!("Successfully loaded scene \"{}\"", load.filename,)
    }
    Ok(())
}

fn serialize_world(spawn_query: &Query<(&SpawnTracker, Option<&Transform>)>) -> Result<String> {
    let objects: Vec<_> = spawn_query
        .iter()
        .filter(|(spawn_tracker, _)| !matches!(spawn_tracker.object, GameObject::Player))
        .map(|(spawn_tracker, transform)| SpawnEvent {
            object: spawn_tracker.object,
            transform: transform.map(Clone::clone).unwrap_or_default(),
        })
        .collect();
    let serialized_level = SerializedLevel(objects);
    ron::ser::to_string_pretty(&serialized_level, default()).context("Failed to serialize world")
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, TypeUuid)]
#[uuid = "eb7cc7bc-5a97-41ed-b0c3-0d4e2137b73b"]
#[reflect(Serialize, Deserialize)]
pub struct SerializedLevel(pub Vec<SpawnEvent>);
