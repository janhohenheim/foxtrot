use std::any::TypeId;

use bevy::{prelude::*, utils::HashSet, window::CursorGrabMode};
use bevy_yarnspinner::events::DialogueStartEvent;

use crate::{screens::Screen, third_party::bevy_yarnspinner::is_dialogue_running};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(CrosshairState, CrosshairTextures)>();

    app.add_systems(
        Update,
        (
            capture_cursor.run_if(not(is_dialogue_running)),
            release_cursor.run_if(on_event::<DialogueStartEvent>),
            update_crosshair,
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
    crosshair: Option<Single<&mut Visibility, With<CrosshairState>>>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Clear Bevy's grab mode cache by setting a different grab mode
        // because an unlocked cursor will not update the current `CursorGrabMode`.
        // See <https://github.com/bevyengine/bevy/issues/8949>
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }
    if let Some(mut crosshair) = crosshair {
        **crosshair = Visibility::Inherited;
    }
}

pub fn release_cursor(
    mut window: Single<&mut Window>,
    crosshair: Option<Single<&mut Visibility, With<CrosshairState>>>,
) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
    if let Some(mut crosshair) = crosshair {
        **crosshair = Visibility::Hidden;
    }
}

/// Show a crosshair for better aiming
fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let crosshair_dot = asset_server.load("ui/crosshair_dot.png");
    let crosshair_square = asset_server.load("ui/crosshair_square.png");
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
                CrosshairState::default(),
                CrosshairTextures {
                    dot: crosshair_dot.clone(),
                    square: crosshair_square.clone(),
                },
                ImageNode::new(crosshair_dot),
            ));
        });
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub(crate) struct CrosshairState {
    pub(crate) wants_square: HashSet<TypeId>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
struct CrosshairTextures {
    dot: Handle<Image>,
    square: Handle<Image>,
}

fn update_crosshair(
    crosshair: Option<
        Single<(&CrosshairState, &CrosshairTextures, &mut ImageNode), Changed<CrosshairState>>,
    >,
) {
    let Some((crosshair_state, crosshair_textures, mut image_node)) =
        crosshair.map(|c| c.into_inner())
    else {
        return;
    };
    if crosshair_state.wants_square.is_empty() {
        image_node.image = crosshair_textures.dot.clone();
    } else {
        image_node.image = crosshair_textures.square.clone();
    }
}
