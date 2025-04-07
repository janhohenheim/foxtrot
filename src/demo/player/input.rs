use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use super::Player;
use super::movement::{Jump, Move};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.add_actions_marker::<Player>() // All contexts should be registered.
        .add_observer(binding); // Add observer to setup bindings.
}

const DEFAULT_SPEED: f32 = 3.0;

// To define mappings for actions, write an observer for `Binding`.
// It's also possible to create bindings before the insertion,
// but this way you can conveniently reload bindings when settings change.
fn binding(trigger: Trigger<Binding<Player>>, mut players: Query<&mut Actions<Player>>) {
    let mut actions = players.get_mut(trigger.entity()).unwrap();

    // Mappings like WASD or sticks are very common,
    // so we provide built-ins to assign all keys/axes at once.
    // We don't assign any conditions and in this case the action will
    // be triggered with any non-zero value.
    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), GamepadStick::Left))
        .with_modifiers((
            DeadZone::default(), // Apply non-uniform normalization to ensure consistent speed, otherwise diagonal movement will be faster.
            SmoothNudge::default(), // Make movement smooth and independent of the framerate. To only make it framerate-independent, use `DeltaScale`.
            Scale::splat(DEFAULT_SPEED), // Additionally multiply by a constant to achieve the desired speed.
        ));

    // Multiple inputs can be assigned to a single action,
    // and the action will respond to any of them.
    actions
        .bind::<Jump>()
        .to((KeyCode::Space, GamepadButton::South));
}
