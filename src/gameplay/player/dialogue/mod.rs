//! Player dialogue handling. This module starts the Yarn Spinner dialogue when the player starts interacting with an NPC.

use std::any::Any;

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::events::Started;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    PostPhysicsAppSystems,
    gameplay::crosshair::CrosshairState,
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
    default_input::{BlocksInput, Interact},
    pickup::is_holding_prop,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPrompt>();

    app.configure_sets(
        Update,
        (
            DialogueSystems::UpdateOpportunity,
            DialogueSystems::UpdateUi,
        )
            .chain()
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );

    app.add_systems(
        Update,
        check_for_dialogue_opportunity
            .in_set(DialogueSystems::UpdateOpportunity)
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
            .in_set(PostPhysicsAppSystems::Update),
    );

    app.add_observer(interact_with_dialogue);

    app.add_plugins(ui::plugin);
}

#[derive(Debug, SystemSet, Hash, Eq, PartialEq, Clone, Copy)]
pub(super) enum DialogueSystems {
    UpdateOpportunity,
    UpdateUi,
}

#[cfg_attr(feature = "hot_patch", hot)]
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

#[cfg_attr(feature = "hot_patch", hot)]
fn interact_with_dialogue(
    _trigger: Trigger<Started<Interact>>,
    mut interaction_prompt: Single<&mut InteractionPrompt>,
    mut dialogue_runner: Single<&mut DialogueRunner>,
    mut crosshair: Single<&mut CrosshairState>,
    mut blocks_input: ResMut<BlocksInput>,
) {
    let Some(node) = interaction_prompt.0.take() else {
        return;
    };
    dialogue_runner.start_node(&node.yarn_node);
    blocks_input.insert(interact_with_dialogue.type_id());
    crosshair
        .wants_free_cursor
        .insert(interact_with_dialogue.type_id());
}

#[cfg_attr(feature = "hot_patch", hot)]
fn restore_input_context(
    mut crosshair: Single<&mut CrosshairState>,
    mut blocks_input: ResMut<BlocksInput>,
) {
    blocks_input.remove(&interact_with_dialogue.type_id());
    crosshair
        .wants_free_cursor
        .remove(&interact_with_dialogue.type_id());
}
