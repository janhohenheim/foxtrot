use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_enhanced_input::events::Started;
use bevy_yarnspinner::prelude::*;

use crate::{
    screens::Screen,
    third_party::{avian3d::CollisionLayer, bevy_yarnspinner::YarnNode},
};

use super::{camera::PlayerCameraParent, input::Interact};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPrompt>();

    app.add_systems(OnEnter(Screen::Gameplay), setup_interaction_prompt);
    app.add_systems(
        Update,
        (check_for_dialogue_opportunity, update_interaction_prompt_ui)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_observer(interact_with_dialogue);
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

fn setup_interaction_prompt(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Interaction Prompt"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                left: Val::Percent(50.0),
                align_items: AlignItems::Center,
                ..default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    left: Val::Px(50.0),
                    ..default()
                },
                Text::new(""),
                Visibility::Hidden,
                InteractionPrompt::default(),
            ));
        });
}

fn update_interaction_prompt_ui(
    dialogue_prompt: Option<
        Single<(&mut Text, &mut Visibility, &InteractionPrompt), Changed<InteractionPrompt>>,
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

fn interact_with_dialogue(
    _trigger: Trigger<Started<Interact>>,
    interaction_prompt: Option<Single<&mut InteractionPrompt>>,
    dialogue_runner: Option<Single<&mut DialogueRunner>>,
) {
    let Some(mut interaction_prompt) = interaction_prompt else {
        return;
    };
    let Some(mut dialogue_runner) = dialogue_runner else {
        return;
    };
    if let Some(node) = &mut interaction_prompt.0 {
        dialogue_runner.start_node(&node.yarn_node);
    }
}
