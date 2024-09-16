use bevy::prelude::*;
use bevy::ui::Val::*;
use sickle_ui::{prelude::*, ui_commands::SetTextExt as _};

use super::{
    OpportunitySystem, {AvailablePlayerInteraction, PlayerInteractionParameters},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_prompt);
    app.add_systems(Update, show_prompt.in_set(OpportunitySystem::ShowPrompt));
}

fn show_prompt(
    q_active_interactable: Query<&AvailablePlayerInteraction, Changed<AvailablePlayerInteraction>>,
    q_interaction_params: Query<&PlayerInteractionParameters>,
    mut q_prompt_text_node: Query<&mut Text, With<PromptTextNode>>,
    mut q_prompt_visibility: Query<&mut Visibility, With<PromptUiRootNode>>,
) {
    let Ok(active_interactable) = q_active_interactable.get_single() else {
        // Nothing changed
        return;
    };
    let Ok(mut prompt_visibility) = q_prompt_visibility.get_single_mut() else {
        return;
    };
    let Some(interaction_params) = active_interactable
        .0
        .and_then(|entity| q_interaction_params.get(entity).ok())
    else {
        // The previous interactable is no longer available.
        // Note that we don't check against previous values for change detection
        // because this system is only run when the active interactable changes in the first place.
        *prompt_visibility = Visibility::Hidden;
        return;
    };
    let Ok(mut prompt_text_node) = q_prompt_text_node.get_single_mut() else {
        return;
    };

    *prompt_visibility = Visibility::Inherited;
    prompt_text_node.sections[0].value = interaction_params.prompt.clone();
}

fn spawn_prompt(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column
                .label(LabelConfig::default())
                .style()
                .bottom(Percent(100. / 3.))
                .entity_commands()
                .set_text("This is label 1.", None)
                .insert(PromptTextNode);
        })
        .style()
        .position_type(PositionType::Absolute)
        .width(Percent(100.0))
        .height(Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::End)
        .visibility(Visibility::Hidden)
        .entity_commands()
        .insert((PromptUiRootNode, Name::new("Prompt UI")));
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct PromptTextNode;

#[derive(Debug, Component, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct PromptUiRootNode;
