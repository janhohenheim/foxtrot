use bevy::prelude::*;

use crate::spawning::{GameObject, SpawnEvent};
use crate::world_serialization::{CurrentLevel, WorldLoadRequest};
use crate::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

fn setup(
    mut commands: Commands,
    mut loader: EventWriter<WorldLoadRequest>,
    mut spawner: EventWriter<SpawnEvent>,
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

    spawner.send(SpawnEvent {
        object: GameObject::Player,
        transform: Transform::from_translation((0., 10., 0.).into()),
        parent: None,
        name: Some("Player".into()),
    })
}
