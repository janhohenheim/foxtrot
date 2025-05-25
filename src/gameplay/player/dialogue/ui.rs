//! The UI part of the dialogue handling. We change the crosshair when we are able to interact with a dialogue. When the dialogue is running, we disable the player's input and disable the cursor.
//! When the dialogue is complete, we restore everything.
//! When a dialogue is able to be started, we signal this to other systems by inserting a `InteractionPrompt`.

use super::{DialogueSystems, InteractionPrompt};
use crate::{PostPhysicsAppSystems, gameplay::crosshair::CrosshairState, screens::Screen};
use bevy::{prelude::*, window::CursorGrabMode};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_yarnspinner::events::{DialogueCompleteEvent, DialogueStartEvent};
use std::any::Any;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_interaction_prompt);
    app.add_systems(
        Update,
        update_interaction_prompt_ui
            .in_set(DialogueSystems::UpdateUi)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        Update,
        (
            hide_crosshair_on_dialogue_start.run_if(on_event::<DialogueStartEvent>),
            show_crosshair_on_dialogue_end.run_if(on_event::<DialogueCompleteEvent>),
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
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
            StateScoped(Screen::Gameplay),
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

#[cfg_attr(feature = "hot_patch", hot)]
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

#[cfg_attr(feature = "hot_patch", hot)]
fn hide_crosshair_on_dialogue_start(
    mut crosshair: Single<&mut CrosshairState>,
    mut window: Single<&mut Window>,
) {
    crosshair
        .wants_invisible
        .insert(hide_crosshair_on_dialogue_start.type_id());
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

#[cfg_attr(feature = "hot_patch", hot)]
fn show_crosshair_on_dialogue_end(
    mut crosshair: Single<&mut CrosshairState>,
    mut window: Single<&mut Window>,
) {
    crosshair
        .wants_invisible
        .remove(&hide_crosshair_on_dialogue_start.type_id());
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}
