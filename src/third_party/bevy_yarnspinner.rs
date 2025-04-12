use bevy::prelude::*;
use bevy_trenchbroom::prelude::BaseClass;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<YarnNode>();

    app.add_plugins((
        YarnSpinnerPlugin::default(),
        ExampleYarnSpinnerDialogueViewPlugin::default(),
    ));
    app.add_systems(
        PreUpdate,
        setup_dialogue_runner.run_if(resource_added::<YarnProject>),
    );
}

fn setup_dialogue_runner(mut commands: Commands, yarn_project: Res<YarnProject>) {
    commands.spawn((
        Name::new("Dialogue Runner"),
        yarn_project.create_dialogue_runner(),
    ));
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
