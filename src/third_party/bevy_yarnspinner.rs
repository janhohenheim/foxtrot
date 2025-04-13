use bevy::prelude::*;
use bevy_trenchbroom::prelude::BaseClass;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<YarnNode>();

    app.add_plugins((
        YarnSpinnerPlugin::default(),
        ExampleYarnSpinnerDialogueViewPlugin::default(),
    ));
    app.add_systems(OnEnter(Screen::Gameplay), setup_dialogue_runner);
    app.add_systems(
        OnExit(Screen::Gameplay),
        abort_all_dialogues_when_leaving_gameplay,
    );
}

fn setup_dialogue_runner(mut commands: Commands, yarn_project: Res<YarnProject>) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        Name::new("Dialogue Runner"),
        yarn_project.create_dialogue_runner(),
    ));
}

fn abort_all_dialogues_when_leaving_gameplay(
    q_dialogue_runner: Query<Entity, With<DialogueRunner>>,
    mut dialogue_complete_events: EventWriter<DialogueCompleteEvent>,
) {
    for dialogue_runner in q_dialogue_runner.iter() {
        dialogue_complete_events.send(DialogueCompleteEvent {
            source: dialogue_runner,
        });
    }
}

pub(crate) fn is_dialogue_running(dialogue_runner: Option<Single<&DialogueRunner>>) -> bool {
    dialogue_runner.is_some_and(|dialogue_runner| dialogue_runner.is_running())
}

#[derive(BaseClass, Component, Debug, Clone, Reflect, Eq, PartialEq)]
#[reflect(Component, Default, Debug)]
pub(crate) struct YarnNode {
    #[no_default]
    pub(crate) yarn_node: String,
    pub(crate) prompt: String,
}

impl YarnNode {
    pub(crate) fn new(yarn_node: impl Into<String>) -> Self {
        Self {
            yarn_node: yarn_node.into(),
            ..default()
        }
    }
}

impl Default for YarnNode {
    fn default() -> Self {
        Self {
            yarn_node: "".to_string(),
            prompt: "Talk".to_string(),
        }
    }
}
