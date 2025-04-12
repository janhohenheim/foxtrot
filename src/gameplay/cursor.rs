use bevy::{prelude::*, window::CursorGrabMode};
use bevy_yarnspinner::events::DialogueStartEvent;

use crate::{screens::Screen, third_party::bevy_yarnspinner::is_dialogue_running};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            capture_cursor.run_if(not(is_dialogue_running)),
            release_cursor.run_if(on_event::<DialogueStartEvent>),
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(OnEnter(Screen::Gameplay), spawn_crosshair);
    app.add_systems(OnExit(Screen::Gameplay), release_cursor);
}

fn capture_cursor(
    mut window: Single<&mut Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Clear Bevy's grab mode cache by setting a different grab mode
        // because an unlocked cursor will not update the current `CursorGrabMode`.
        // See <https://github.com/bevyengine/bevy/issues/8949>
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }
}

pub fn release_cursor(mut window: Single<&mut Window>) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

/// Show a crosshair for better aiming
fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let crosshair_texture = asset_server.load("ui/crosshair.png");
    commands
        .spawn((
            Name::new("Crosshair"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Crosshair Image"),
                ImageNode::new(crosshair_texture),
            ));
        });
}
