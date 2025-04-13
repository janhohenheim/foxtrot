use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::{events::Started, prelude::Actions};
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    screens::Screen,
    third_party::{
        avian3d::CollisionLayer,
        bevy_yarnspinner::{YarnNode, is_dialogue_running},
    },
};

mod ui;

use super::{
    Player,
    camera::PlayerCameraParent,
    default_input::{DefaultInputContext, Interact},
    pickup::is_holding_prop,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPrompt>();

    app.configure_sets(
        Update,
        (DialogueSet::UpdateOpportunity, DialogueSet::UpdateUI).chain(),
    );

    app.add_systems(
        Update,
        check_for_dialogue_opportunity
            .param_warn_once()
            .in_set(DialogueSet::UpdateOpportunity)
            .run_if(in_state(Screen::Gameplay))
            .run_if(not(is_dialogue_running))
            .run_if(not(is_holding_prop)),
    );
    app.add_systems(
        Update,
        restore_input_context
            .param_warn_once()
            .run_if(in_state(Screen::Gameplay))
            .run_if(on_event::<DialogueCompleteEvent>),
    );

    app.add_observer(interact_with_dialogue.param_warn_once());

    app.add_plugins(ui::plugin);
}

#[derive(Debug, SystemSet, Hash, Eq, PartialEq, Clone, Copy)]
pub(super) enum DialogueSet {
    UpdateOpportunity,
    UpdateUI,
}

fn check_for_dialogue_opportunity(
    player: Single<&GlobalTransform, With<PlayerCameraParent>>,
    mut interaction_prompt: Single<&mut InteractionPrompt>,
    q_yarn_node: Query<&YarnNode>,
    spatial_query: SpatialQuery,
) {
    let camera_transform = player.compute_transform();
    const MAX_INTERACTION_DISTANCE: f32 = 3.0;
    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        camera_transform.forward(),
        MAX_INTERACTION_DISTANCE,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Default),
    );
    let node = hit
        .and_then(|hit| q_yarn_node.get(hit.entity).ok())
        .cloned();
    if interaction_prompt.0 != node {
        interaction_prompt.0 = node;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
struct InteractionPrompt(Option<YarnNode>);

fn interact_with_dialogue(
    _trigger: Trigger<Started<Interact>>,
    mut commands: Commands,
    mut interaction_prompt: Single<&mut InteractionPrompt>,
    mut dialogue_runner: Single<&mut DialogueRunner>,
    player: Single<Entity, With<Player>>,
) {
    let Some(node) = interaction_prompt.0.take() else {
        return;
    };
    dialogue_runner.start_node(&node.yarn_node);
    commands
        .entity(*player)
        .remove::<Actions<DefaultInputContext>>();
}

fn restore_input_context(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands
        .entity(*player)
        .insert(Actions::<DefaultInputContext>::default());
}
