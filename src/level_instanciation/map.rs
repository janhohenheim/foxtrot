use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instanciation::spawning::{DelayedSpawnEvent, GameObject, SpawnEvent};
use crate::GameState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

fn setup(
    mut commands: Commands,
    mut loader: EventWriter<WorldLoadRequest>,
    mut delayed_spawner: EventWriter<DelayedSpawnEvent>,
    current_level: Option<Res<CurrentLevel>>,
) {
    if current_level.is_some() {
        return;
    }
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.1,
    });

    loader.send(WorldLoadRequest {
        filename: "demo".to_string(),
    });

    // Make sure the player is spawned after the level
    delayed_spawner.send(DelayedSpawnEvent {
        tick_delay: 2,
        event: SpawnEvent {
            object: GameObject::Player,
            transform: Transform::from_translation((0., 0.5, 0.).into()),
            parent: None,
            name: Some("Player".into()),
        },
    });
}
