use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

pub mod conditions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::default(),
        ExampleYarnSpinnerDialogueViewPlugin::default(),
    ));
    app.observe(start_dialogue);
    app.add_systems(
        PreUpdate,
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
    );
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

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    commands.spawn((
        Name::new("Dialogue Runner"),
        project.create_dialogue_runner(),
    ));
}
