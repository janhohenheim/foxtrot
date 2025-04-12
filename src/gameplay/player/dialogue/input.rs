use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<DialogueInputContext>();
    app.add_observer(dialogue_binding);
}

#[derive(Debug, InputContext, Default)]
pub(crate) struct DialogueInputContext;

fn dialogue_binding(
    trigger: Trigger<Binding<DialogueInputContext>>,
    mut players: Query<&mut Actions<DialogueInputContext>>,
) {
    let mut actions = players.get_mut(trigger.entity()).unwrap();
    actions
        .bind::<MoveSelection>()
        .to((
            Cardinal::wasd_keys(),
            Cardinal::arrow_keys(),
            Cardinal::dpad_buttons(),
            GamepadStick::Left,
        ))
        .with_modifiers(Scale::new(Vec3::new(0.0, 1.0, 0.0)));

    actions
        .bind::<WriteFaster>()
        .to((KeyCode::KeyE, KeyCode::Space, GamepadButton::South));
    actions
        .bind::<AdvanceDialogue>()
        .to((KeyCode::KeyE, KeyCode::Space, GamepadButton::South));
}

#[derive(Debug, InputAction)]
#[input_action(output = Vec2, require_reset = true)]
pub(crate) struct MoveSelection;

#[derive(Debug, InputAction)]
#[input_action(output = bool, require_reset = true)]
pub(crate) struct WriteFaster;

#[derive(Debug, InputAction)]
#[input_action(output = bool, require_reset = true)]
pub(crate) struct AdvanceDialogue;
