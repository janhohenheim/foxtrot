use crate::level_instanciation::spawning::{GameObject, SpawnEvent, SpawnTracker};
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionUi;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, iter};
use crate::file_system_interaction::asset_loading::LevelAssets;

pub struct WorldSerializationPlugin;

impl Plugin for WorldSerializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldSaveRequest>()
            .add_event::<WorldLoadRequest>()
            .add_system(save_world.after("spawn_requested"))
            .add_system_to_stage(CoreStage::PostUpdate, load_world);
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
    spawn_query: Query<(&SpawnTracker, &Name, Option<&Parent>, Option<&Transform>)>,
) {
    for save in save_requests.iter() {
        let scene = save.filename.clone();
        let valid_candidates: Vec<_> = iter::once(scene.clone())
            .chain((1..).map(|n| format!("{0}-{n}", scene.clone())))
            .map(|filename| {
                Path::new("assets")
                    .join("levels")
                    .join(format!("{filename}.scn.ron"))
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
            let serialized_world = serialize_world(&spawn_query);
            fs::create_dir_all(path.parent().unwrap()).expect("Failed to create scene directory");
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
) {
    let level_handles = match level_handles {
        Some(level_handles) => level_handles,
        None => {
            return;
        }
    };
    for load in load_requests.iter() {
        let path = format!("levels/{}.lvl.ron", load.filename);
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
        let spawn_events = &levels.get(handle).unwrap().0;
        for entity in &current_spawn_query {
            commands
                .get_entity(entity)
                .unwrap_or_else(|| panic!("Failed to get entity while loading"))
                .despawn_recursive();
        }
        for event in spawn_events {
            spawn_requests.send(event.clone());
        }
        commands.insert_resource(CurrentLevel {
            scene: load.filename.clone(),
        });
        commands.init_resource::<InteractionUi>();
        commands.init_resource::<ActiveConditions>();
        commands.remove_resource::<CurrentDialog>();

        info!("Successfully loaded scene \"{}\"", load.filename,)
    }
}

fn serialize_world(
    spawn_query: &Query<(&SpawnTracker, &Name, Option<&Parent>, Option<&Transform>)>,
) -> String {
    let objects: Vec<_> = spawn_query
        .iter()
        .filter(|(spawn_tracker, _, _, _)| !matches!(spawn_tracker.object, GameObject::Player))
        .map(|(spawn_tracker, name, parent, transform)| {
            let parent = parent
                .and_then(|parent| spawn_query.get(parent.get()).ok())
                .and_then(|(spawn_tracker, name, _, _)| {
                    (spawn_tracker.get_default_name() != name.as_str())
                        .then(|| name.to_string().into())
                });
            SpawnEvent {
                object: spawn_tracker.object,
                transform: transform.map(Clone::clone).unwrap_or_default(),
                name: Some(String::from(name).into()),
                parent,
            }
        })
        .collect();
    let serialized_level = SerializedLevel(objects);
    ron::to_string(&serialized_level).expect("Failed to serialize world")
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, TypeUuid)]
#[uuid = "eb7cc7bc-5a97-41ed-b0c3-0d4e2137b73b"]
#[reflect(Serialize, Deserialize)]
pub struct SerializedLevel(pub Vec<SpawnEvent>);
