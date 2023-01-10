use bevy::prelude::*;

use crate::world_serialization::LoadRequest;
use crate::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

fn setup(mut commands: Commands, mut loader: EventWriter<LoadRequest>) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.1,
    });

    loader.send(LoadRequest {
        filename: "demo".to_string(),
    });
}
