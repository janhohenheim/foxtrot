use crate::player::Player;
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
        (&Transform, &DialogSensor, &CollidingEntities),
        Changed<CollidingEntities>,
    >,
    q_player: Query<&Transform, With<Player>>,
) {
    for (dialog_transform, sensor, colliding_entities) in &q_dialog_sensor {
        let Some(player_transform) = colliding_entities
            .iter()
            .find_map(|entity| q_player.get(*entity).ok())
        else {
            continue;
        };
    }
}
