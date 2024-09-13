use crate::player::{camera::PlayerCamera, Player};
use crate::system_set::VariableGameSet;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use conditions::dialog_running;

pub mod conditions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::default(),
        ExampleYarnSpinnerDialogueViewPlugin::default(),
    ));
    app.add_systems(
        Update,
        on_dialog_collided
            .run_if(not(dialog_running))
            .in_set(VariableGameSet::Dialog),
    );
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component)]
pub struct DialogSensor {
    node: String,
    prompt: String,
}

fn on_dialog_collided(
    q_dialog_sensor: Query<
        (&Position, &DialogSensor, &CollidingEntities),
        Changed<CollidingEntities>,
    >,
    q_player: Query<(), With<Player>>,
    q_camera: Query<&Transform, With<PlayerCamera>>,
) {
    for (dialog_position, sensor, colliding_entities) in &q_dialog_sensor {
        if !colliding_entities
            .iter()
            .any(|entity| q_player.contains(*entity))
        {
            continue;
        };
    }
}
