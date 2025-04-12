use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::{events::Started, prelude::Actions};
use bevy_yarnspinner::prelude::*;

use crate::{
    screens::Screen,
    third_party::{avian3d::CollisionLayer, bevy_yarnspinner::YarnNode},
};

mod ui;

use super::{
    Player,
    camera::PlayerCameraParent,
    default_input::{DefaultInputContext, Interact},
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
            .in_set(DialogueSet::UpdateOpportunity)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_observer(interact_with_dialogue);

    app.add_plugins(ui::plugin);
}

#[derive(Debug, SystemSet, Hash, Eq, PartialEq, Clone, Copy)]
pub(super) enum DialogueSet {
    UpdateOpportunity,
    UpdateUI,
}

fn check_for_dialogue_opportunity(
    player: Option<Single<&GlobalTransform, With<PlayerCameraParent>>>,
    interaction_prompt: Option<Single<&mut InteractionPrompt>>,
    q_yarn_node: Query<&YarnNode>,
    spatial_query: SpatialQuery,
) {
    let Some(player) = player else {
        return;
    };
    let Some(mut interaction_prompt) = interaction_prompt else {
        return;
    };
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
    interaction_prompt: Option<Single<&mut InteractionPrompt>>,
    dialogue_runner: Option<Single<&mut DialogueRunner>>,
    player: Option<Single<Entity, With<Player>>>,
) {
    let Some(mut interaction_prompt) = interaction_prompt else {
        return;
    };
    let Some(mut dialogue_runner) = dialogue_runner else {
        return;
    };
    let Some(player) = player else {
        return;
    };
    let Some(node) = &mut interaction_prompt.0 else {
        return;
    };
    dialogue_runner.start_node(&node.yarn_node);
    commands
        .entity(*player)
        .remove::<Actions<DefaultInputContext>>();
}
