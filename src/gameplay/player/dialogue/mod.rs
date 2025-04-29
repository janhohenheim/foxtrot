//! Player dialogue handling. This module starts the Yarn Spinner dialogue when the player starts interacting with an NPC.

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::{events::Started, prelude::Actions};
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    AppSet,
    screens::Screen,
    third_party::{
        avian3d::CollisionLayer,
        bevy_yarnspinner::{YarnNode, is_dialogue_running},
    },
};

mod ui;

use super::{
    Player,
    camera::PlayerCamera,
    default_input::{DefaultInputContext, Interact},
    pickup::is_holding_prop,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPrompt>();

    app.configure_sets(
        Update,
        (DialogueSet::UpdateOpportunity, DialogueSet::UpdateUi)
            .chain()
            .in_set(AppSet::ChangeUi),
    );

    app.add_systems(
        Update,
        check_for_dialogue_opportunity

            .in_set(DialogueSet::UpdateOpportunity)
            .run_if(
                in_state(Screen::Gameplay)
                    .and(not(is_dialogue_running))
                    .and(not(is_holding_prop)),
            ),
    );
    app.add_systems(
        Update,
        restore_input_context

            .run_if(in_state(Screen::Gameplay).and(on_event::<DialogueCompleteEvent>))
            .in_set(AppSet::Update),
    );

    app.add_observer(interact_with_dialogue);

    app.add_plugins(ui::plugin);
}

#[derive(Debug, SystemSet, Hash, Eq, PartialEq, Clone, Copy)]
pub(super) enum DialogueSet {
    UpdateOpportunity,
    UpdateUi,
}

fn check_for_dialogue_opportunity(
    player: Single<&GlobalTransform, With<PlayerCamera>>,
    player_collider: Single<Entity, With<Player>>,
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
        &SpatialQueryFilter::from_mask(CollisionLayer::Character)
            .with_excluded_entities([*player_collider]),
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
