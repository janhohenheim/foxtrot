use crate::player_control::{actions::ActionsFrozen, camera::IngameCamera};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use bevy_yarnspinner_example_dialogue_view::{prelude::*, UiRootNode};
use leafwing_input_manager::plugin::InputManagerSystem;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EguiPlugin,
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .add_systems(
        Update,
        (
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
            unfreeze_after_dialog.after(InputManagerSystem::ManualControl),
            set_ui_target_camera,
        )
            .after(ExampleYarnSpinnerDialogueViewSystemSet),
    )
    .init_resource::<CurrentDialogTarget>()
    .register_type::<YarnNode>()
    .register_type::<CurrentDialogTarget>();
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct YarnNode(pub(crate) String);

#[derive(Resource, Debug, Clone, Copy, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct CurrentDialogTarget(pub(crate) Option<Entity>);

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let dialogue_runner = project.create_dialogue_runner();
    // Immediately start showing the dialogue to the player
    commands.spawn(dialogue_runner);
}

fn unfreeze_after_dialog(
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
    mut dialog_target: ResMut<CurrentDialogTarget>,
    mut freeze: ResMut<ActionsFrozen>,
) {
    for _event in dialogue_complete_event.read() {
        dialog_target.0 = None;
        freeze.unfreeze();
    }
}

fn set_ui_target_camera(
    mut commands: Commands,
    root_ui_node: Query<Entity, (With<UiRootNode>, Without<TargetCamera>)>,
    main_camera: Query<Entity, With<IngameCamera>>,
) {
    for camera_entity in main_camera.iter() {
        for node_entity in root_ui_node.iter() {
            commands
                .entity(node_entity)
                .insert(TargetCamera(camera_entity));
        }
    }
}
