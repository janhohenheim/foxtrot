//! The UI part of the dialogue handling. We change the crosshair when we are able to interact with a dialogue. When the dialogue is running, we disable the player's input and disable the cursor.
//! When the dialogue is complete, we restore everything.
//! When a dialogue is able to be started, we signal this to other systems by inserting a `InteractionPrompt`.

use super::{DialogueSystems, InteractionPrompt};
use crate::{gameplay::crosshair::CrosshairState, screens::Screen};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use bevy_yarnspinner::events::{DialogueCompleted, DialogueStarted};
use std::any::Any;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_interaction_prompt);
    app.add_systems(
        Update,
        update_interaction_prompt_ui
            .in_set(DialogueSystems::UpdateUi)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_observer(hide_crosshair_on_dialogue_start)
        .add_observer(show_crosshair_on_dialogue_end);
}

pub(crate) fn setup_interaction_prompt(mut commands: Commands) {
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
            DespawnOnExit(Screen::Gameplay),
            Pickable::IGNORE,
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
    dialogue_prompt: Single<(&mut Text, &mut Visibility, Ref<InteractionPrompt>)>,
    mut crosshair: Single<&mut CrosshairState>,
) {
    let (mut text, mut prompt_visibility, dialogue_prompt) = dialogue_prompt.into_inner();
    if !dialogue_prompt.is_changed() {
        return;
    }

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

fn hide_crosshair_on_dialogue_start(
    _start: On<DialogueStarted>,
    mut crosshair: Single<&mut CrosshairState>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    crosshair
        .wants_invisible
        .insert(hide_crosshair_on_dialogue_start.type_id());
    cursor_options.grab_mode = CursorGrabMode::None;
}

fn show_crosshair_on_dialogue_end(
    _complete: On<DialogueCompleted>,
    mut crosshair: Single<&mut CrosshairState>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    crosshair
        .wants_invisible
        .remove(&hide_crosshair_on_dialogue_start.type_id());
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
