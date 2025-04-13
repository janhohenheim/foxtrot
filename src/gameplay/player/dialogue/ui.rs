use std::any::Any;

use bevy::prelude::*;

use crate::{gameplay::cursor::CrosshairState, screens::Screen};

use super::{DialogueSet, InteractionPrompt};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_interaction_prompt);
    app.add_systems(
        Update,
        update_interaction_prompt_ui
            .in_set(DialogueSet::UpdateUI)
            .run_if(in_state(Screen::Gameplay)),
    );
}

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
    crosshair: Option<Single<&mut CrosshairState>>,
) {
    let Some((mut text, mut prompt_visibility, dialogue_prompt)) =
        dialogue_prompt.map(|d| d.into_inner())
    else {
        return;
    };
    let Some(mut crosshair) = crosshair.map(|c| c.into_inner()) else {
        return;
    };
    let system_id = update_interaction_prompt_ui.type_id();
    if let Some(node) = &dialogue_prompt.0 {
        text.0 = format!("E: {}", node.prompt);
        *prompt_visibility = Visibility::Inherited;
        crosshair.wants_square.insert(system_id);
    } else {
        text.0 = String::new();
        *prompt_visibility = Visibility::Hidden;
        crosshair.wants_square.remove(&system_id);
    }
}
