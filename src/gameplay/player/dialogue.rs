use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::{
    screens::Screen,
    third_party::{avian3d::CollisionLayer, bevy_yarnspinner::YarnNode},
};

use super::camera::PlayerCameraParent;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DialoguePrompt>();

    app.add_systems(OnEnter(Screen::Gameplay), setup_dialogue_prompt);
    app.add_systems(
        Update,
        (check_for_dialogue_opportunity, update_dialogue_prompt_ui)
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
        dialogue_prompt.0 = node;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
struct DialoguePrompt(Option<YarnNode>);

fn setup_dialogue_prompt(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Dialogue Prompt"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_child((
            Node {
                left: Val::Px(40.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            Text::new(""),
            Visibility::Hidden,
            DialoguePrompt::default(),
        ));
}

fn update_dialogue_prompt_ui(
    dialogue_prompt: Option<
        Single<(&mut Text, &mut Visibility, &DialoguePrompt), Changed<DialoguePrompt>>,
    >,
) {
    let Some((mut text, mut visibility, dialogue_prompt)) = dialogue_prompt.map(|d| d.into_inner())
    else {
        return;
    };
    if let Some(node) = &dialogue_prompt.0 {
        text.0 = format!("E: {}", node.prompt);
        *visibility = Visibility::Inherited;
    } else {
        text.0 = String::new();
        *visibility = Visibility::Hidden;
    }
}
