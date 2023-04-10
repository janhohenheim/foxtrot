use crate::file_system_interaction::asset_loading::LevelAssets;
use crate::level_instantiation::spawning::GameObject;
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionOpportunities;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};
use spew::prelude::*;
use std::path::Path;
use std::{fs, iter};

pub(crate) fn level_serialization_plugin(app: &mut App) {
    app.add_event::<WorldSaveRequest>()
        .add_event::<WorldLoadRequest>()
        .add_systems(
            (
                save_world,
                load_world.run_if(resource_exists::<LevelAssets>()),
            )
                .in_base_set(CoreSet::PostUpdate),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct WorldSaveRequest {
    pub(crate) filename: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct WorldLoadRequest {
    pub(crate) filename: String,
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct CurrentLevel {
    pub(crate) scene: String,
}

#[sysfail(log(level = "error"))]
fn save_world(
    mut save_requests: EventReader<WorldSaveRequest>,
    spawn_query: Query<(&GameObject, Option<&Transform>)>,
) -> Result<()> {
    for save in save_requests.iter() {
        let scene = save.filename.clone();
        let valid_candidates: Vec<_> = iter::once(scene.clone())
            .chain((1..).map(|n| format!("{0}-{n}", scene.clone())))
            .map(|filename| {
                Path::new("assets")
                    .join("levels")
                    .join(filename)
                    .with_extension("lvl.ron")
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
pub(crate) struct Protected;

#[sysfail(log(level = "error"))]
fn load_world(
    mut commands: Commands,
    mut load_requests: EventReader<WorldLoadRequest>,
    current_spawn_query: Query<Entity, With<GameObject>>,
    mut spawn_requests: EventWriter<SpawnEvent<GameObject, Transform>>,
    levels: Res<Assets<SerializedLevel>>,
    level_handles: Res<LevelAssets>,
) -> Result<()> {
    for load in load_requests.iter() {
        let path = Path::new("levels")
            .join(load.filename.clone())
            .with_extension("lvl.ron")
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
            .context("Failed to get level from handle in level assets")?;
        let spawn_events = Vec::<SpawnEvent<GameObject, Transform>>::from(*spawn_events);
        for entity in &current_spawn_query {
            commands
                .get_entity(entity)
                .context("Failed to get entity while loading")?
                .despawn_recursive();
        }
        for event in spawn_events.into_iter() {
            spawn_requests.send(event);
        }
        commands.insert_resource(CurrentLevel {
            scene: load.filename.clone(),
        });
        commands.insert_resource(InteractionOpportunities::default());
        commands.insert_resource(ActiveConditions::default());
        commands.remove_resource::<CurrentDialog>();

        info!("Successfully loaded scene \"{}\"", load.filename,)
    }
    Ok(())
}

fn serialize_world(spawn_query: &Query<(&GameObject, Option<&Transform>)>) -> Result<String> {
    let objects: Vec<_> = spawn_query
        .iter()
        .filter(|(game_object, _)| **game_object != GameObject::Player)
        .map(|(game_object, transform)| {
            SpawnEvent::with_data(
                *game_object,
                transform.map(Clone::clone).unwrap_or_default(),
            )
        })
        .collect();
    let serialized_level = SerializedLevel::from(objects);
    ron::ser::to_string_pretty(&serialized_level, default()).context("Failed to serialize world")
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, TypeUuid, Deref, DerefMut)]
#[uuid = "eb7cc7bc-5a97-41ed-b0c3-0d4e2137b73b"]
#[reflect(Serialize, Deserialize)]
pub(crate) struct SerializedLevel(pub(crate) Vec<(GameObject, Transform)>);

impl From<Vec<SpawnEvent<GameObject, Transform>>> for SerializedLevel {
    fn from(events: Vec<SpawnEvent<GameObject, Transform>>) -> Self {
        Self(
            events
                .into_iter()
                .map(|event| (event.object, event.data))
                .collect(),
        )
    }
}

impl From<&SerializedLevel> for Vec<SpawnEvent<GameObject, Transform>> {
    fn from(level: &SerializedLevel) -> Self {
        level
            .iter()
            .map(|(object, transform)| SpawnEvent::with_data(*object, *transform))
            .collect()
    }
}
