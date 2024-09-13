use bevy::prelude::*;
use bevy_yarnspinner::prelude::{DialogueRunner, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

pub mod conditions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::default(),
        ExampleYarnSpinnerDialogueViewPlugin::default(),
    ));
    app.observe(start_dialogue);
}

/// Event triggered to start a dialogue with the targeted entity.
#[derive(Debug, Clone, Event)]
pub struct StartDialog(pub String);

fn start_dialogue(
    trigger: Trigger<StartDialog>,
    mut q_dialogue_runner: Query<&mut DialogueRunner>,
) {
    for mut dialogue_runner in &mut q_dialogue_runner {
        dialogue_runner.start_node(&trigger.event().0);
    }
}
