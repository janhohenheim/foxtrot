use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::condition::ActiveConditions;
use crate::world_interaction::dialog::{CurrentDialog, DialogEvent};
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use chrono::prelude::Local;
use glob::glob;
use serde::{Deserialize, Serialize};
use spew::prelude::*;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn game_state_serialization_plugin(app: &mut App) {
    app.add_event::<GameSaveRequest>()
        .add_event::<GameLoadRequest>()
        .add_systems(
            (
                handle_load_requests,
                handle_save_requests.run_if(resource_exists::<CurrentLevel>()),
            )
                .chain()
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Serialize, Deserialize, Default)]
pub(crate) struct GameSaveRequest {
    pub(crate) filename: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Serialize, Deserialize, Default)]
pub(crate) struct GameLoadRequest {
    pub(crate) filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Resource, Serialize, Deserialize, Default)]
struct SaveModel {
    scene: String,
    #[serde(default, skip_serializing_if = "ActiveConditions::is_empty")]
    conditions: ActiveConditions,
    player_transform: Transform,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dialog_event: Option<DialogEvent>,
}

#[sysfail(log(level = "error"))]
fn handle_load_requests(
    mut commands: Commands,
    mut load_events: EventReader<GameLoadRequest>,
    mut loader: EventWriter<WorldLoadRequest>,
    mut spawner: EventWriter<SpawnEvent<GameObject, Transform>>,
    mut dialog_event_writer: EventWriter<DialogEvent>,
) -> Result<()> {
    for load in load_events.iter() {
        let path = match load
            .filename
            .as_ref()
            .map(|filename| anyhow::Ok(Some(get_save_path(filename.clone()))))
            .unwrap_or_else(|| {
                let mut saves: Vec<_> = glob("./saves/*.sav.ron")
                    .context("Failed to read glob pattern")?
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.is_file())
                    .collect();
                saves.sort_by_cached_key(|f| {
                    f.metadata()
                        .expect("Failed to read file metadata")
                        .modified()
                        .expect("Failed to read file modified time")
                });
                Ok(saves.last().map(|entry| entry.to_owned()))
            })? {
            Some(path) => path,
            None => {
                error!("Failed to load save: No filename provided and no saves found on disk");
                continue;
            }
        };
        let serialized = match fs::read_to_string(&path) {
            Ok(serialized) => {
                info!("Successfully read save at {}", path.to_string_lossy());
                serialized
            }
            Err(e) => {
                error!(
                    "Failed to read save {:?} at {:?}: {}",
                    &load.filename, path, e
                );
                continue;
            }
        };
        let save_model: SaveModel = match ron::from_str(&serialized) {
            Ok(save_model) => save_model,
            Err(e) => {
                error!(
                    "Failed to deserialize save {:?} at {:?}: {}",
                    &load.filename, path, e
                );
                continue;
            }
        };
        loader.send(WorldLoadRequest {
            filename: save_model.scene,
        });
        if let Some(dialog_event) = save_model.dialog_event {
            dialog_event_writer.send(dialog_event);
        }
        commands.insert_resource(save_model.conditions);

        spawner.send(
            SpawnEvent::with_data(GameObject::Player, save_model.player_transform).delay_frames(2),
        );
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
fn handle_save_requests(
    mut save_events: EventReader<GameSaveRequest>,
    conditions: Res<ActiveConditions>,
    dialog: Option<Res<CurrentDialog>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    current_level: Res<CurrentLevel>,
) -> Result<()> {
    let dialog = dialog.map(|dialog| dialog.clone());
    for save in save_events.iter() {
        for player in &player_query {
            let dialog_event = dialog.clone().map(|dialog| DialogEvent {
                dialog: dialog.id,
                source: dialog.source,
                page: Some(dialog.current_page),
            });
            let save_model = SaveModel {
                scene: current_level.scene.clone(),
                conditions: conditions.clone(),
                dialog_event,
                player_transform: player.compute_transform(),
            };
            let serialized = match ron::to_string(&save_model) {
                Ok(string) => string,
                Err(e) => {
                    error!("Failed to save world: {}", e);
                    continue;
                }
            };
            let filename = save
                .filename
                .clone()
                .unwrap_or_else(|| Local::now().to_rfc2822().replace(':', "-"));
            let path = get_save_path(filename.clone());
            let dir = path.parent().context("Failed to get save directory")?;
            fs::create_dir_all(dir).context("Failed to create save directory")?;
            fs::write(&path, serialized)
                .unwrap_or_else(|e| error!("Failed to write save {filename}: {e}"));

            info!("Successfully saved game at {}", path.to_string_lossy());
        }
    }
    Ok(())
}

fn get_save_path(filename: impl Into<Cow<'static, str>>) -> PathBuf {
    let filename = filename.into().to_string();
    Path::new("saves").join(filename).with_extension("sav.ron")
}
