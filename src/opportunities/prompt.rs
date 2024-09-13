use bevy::prelude::*;
use bevy::ui::Val::*;
use sickle_ui::{prelude::*, ui_commands::SetTextExt as _};

use super::{
    available_opportunities::{AvailableOpportunities, OpportunitySensor},
    OpportunitySystem,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_prompt);
    app.add_systems(Update, show_prompt.in_set(OpportunitySystem::ShowPrompt));
}

fn show_prompt(
    mut q_available_opportunities: Query<
        &mut AvailableOpportunities,
        Changed<AvailableOpportunities>,
    >,
    q_opportunity_sensor: Query<&OpportunitySensor>,
    mut q_prompt_text_node: Query<&mut Text, With<PromptTextNode>>,
    mut q_prompt_ui_node: Query<&mut Visibility, With<PromptUiNode>>,
) {
    for mut opportunities in &mut q_available_opportunities {
        let Ok(mut visibility) = q_prompt_ui_node.get_single_mut() else {
            continue;
        };
        let Some(opportunity) = opportunities.pick_one() else {
            *visibility = Visibility::Hidden;
            continue;
        };
        let Ok(sensor) = q_opportunity_sensor.get(opportunity) else {
            // Looks like the opportunity despawned.
            opportunities.remove(&opportunity);
            *visibility = Visibility::Hidden;
            continue;
        };
        let Ok(mut text) = q_prompt_text_node.get_single_mut() else {
            continue;
        };
        text.sections[0].value = sensor.prompt.clone();
        *visibility = Visibility::Inherited;
    }
}

fn spawn_prompt(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column
                .label(LabelConfig::default())
                .entity_commands()
                .set_text("This is label 1.", None)
                .insert(PromptTextNode);
        })
        .style()
        .position_type(PositionType::Absolute)
        .bottom(Percent(1. / 3.))
        .height(Val::Auto);
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct PromptUiNode;

#[derive(Debug, Component, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct PromptTextNode;
