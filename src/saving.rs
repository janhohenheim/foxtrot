use crate::dialog::{ActiveConditions, CurrentDialog};
use crate::player::Player;
use crate::spawning::{GameObject, SpawnEvent};
use crate::world_serialization::{CurrentLevel, WorldLoadRequest};
use crate::GameState;
use bevy::prelude::*;
use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SavingPlugin;

impl Plugin for SavingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameSaveRequest>()
            .add_event::<GameLoadRequest>()
            .register_type::<GameSaveRequest>()
            .register_type::<GameLoadRequest>()
            .add_system_set(
                SystemSet::on_in_stack_update(GameState::Playing)
                    .with_system(handle_load_requests.label("handle_game_load_requests"))
                    .with_system(handle_save_requests.after("handle_game_load_requests")),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct GameSaveRequest {
    filename: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct GameLoadRequest {
    filename: String,
}

#[derive(Debug, Clone, PartialEq, Resource, Serialize, Deserialize, Default)]
struct SaveModel {
    scene: String,
    conditions: ActiveConditions,
    player_transform: Transform,
    current_dialog: Option<CurrentDialog>,
}

fn handle_load_requests(
    mut commands: Commands,
    mut load_events: EventReader<GameLoadRequest>,
    mut loader: EventWriter<WorldLoadRequest>,
    mut spawner: EventWriter<SpawnEvent>,
) {
    for load in load_events.iter() {
        let path = get_save_path(load.filename.clone());
        let serialized = match fs::read_to_string(&path) {
            Ok(serialized) => serialized,
            Err(e) => {
                error!(
                    "Failed to read save {} at {:?}: {}",
                    &load.filename, path, e
                );
                continue;
            }
        };
        let save_model: SaveModel = match ron::from_str(&serialized) {
            Ok(save_model) => save_model,
            Err(e) => {
                error!(
                    "Failed to deserialize save {} at {:?}: {}",
                    &load.filename, path, e
                );
                continue;
            }
        };
        loader.send(WorldLoadRequest {
            filename: load.filename.clone(),
        });
        if let Some(current_dialog) = save_model.current_dialog {
            commands.insert_resource(current_dialog);
        }
        commands.insert_resource(save_model.conditions);

        spawner.send(SpawnEvent {
            object: GameObject::Player,
            transform: save_model.player_transform,
            parent: None,
            name: Some("Player".into()),
        })
    }
}

fn handle_save_requests(
    mut save_events: EventReader<GameSaveRequest>,
    conditions: Res<ActiveConditions>,
    dialog: Option<Res<CurrentDialog>>,
    player_query: Query<&Transform, With<Player>>,
    current_level: Option<Res<CurrentLevel>>,
) {
    let dialog = if let Some(ref dialog) = dialog {
        let dialog: CurrentDialog = dialog.as_ref().clone();
        Some(dialog)
    } else {
        None
    };
    let current_level = match current_level {
        Some(level) => level,
        None => return,
    };
    for save in save_events.iter() {
        for player in &player_query {
            let save_model = SaveModel {
                scene: current_level.scene.clone(),
                conditions: conditions.clone(),
                current_dialog: dialog.clone(),
                player_transform: player.clone(),
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
                .unwrap_or_else(|| Local::now().to_rfc2822().replace(":", "-"));
            let path = get_save_path(filename.clone());
            fs::write(path, serialized)
                .unwrap_or_else(|e| error!("Failed to write save {filename}: {e}"));
        }
    }
}

fn get_save_path(filename: impl Into<Cow<'static, str>>) -> PathBuf {
    Path::new("saves").join(format!("{}.sav.ron", filename.into()))
}
