use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::{
    screens::Screen,
    third_party::{avian3d::CollisionLayer, bevy_yarnspinner::YarnNode},
};

use super::camera::PlayerCameraParent;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_dialogue_prompt);
    app.add_systems(
        Update,
        (check_for_dialogue_opportunity, update_dialogue_prompt)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}

fn check_for_dialogue_opportunity(
    player: Option<Single<&GlobalTransform, With<PlayerCameraParent>>>,
    dialogue_prompt: Option<Single<&mut DialoguePrompt>>,
    q_yarn_node: Query<&YarnNode>,
    spatial_query: SpatialQuery,
) {
    let Some(player) = player else {
        return;
    };
    let Some(mut dialogue_prompt) = dialogue_prompt else {
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
    if dialogue_prompt.0 != node {
        println!("Dialogue opportunity: {:?}", node);
        dialogue_prompt.0 = node;
    }
}

#[derive(Component)]
struct DialoguePrompt(Option<YarnNode>);

fn setup_dialogue_prompt(mut commands: Commands) {
    commands.spawn((
        Name::new("Dialogue Prompt"),
        Node::default(),
        Text::new(""),
        DialoguePrompt(None),
    ));
}

fn update_dialogue_prompt(
    mut dialogue_prompt: Option<Single<&mut DialoguePrompt, Changed<DialoguePrompt>>>,
) {
    let Some(mut dialogue_prompt) = dialogue_prompt else {
        return;
    };
    if dialogue_prompt.0.is_some() {
        // TODO: show dialogue prompt
    } else {
        // TODO: hide dialogue prompt
    }
}
