use bevy::{prelude::*, window::CursorGrabMode};
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

use crate::screens::gameplay::GameplayState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<CursorAction>::default())
        .init_resource::<ActionState<CursorAction>>()
        .insert_resource(CursorAction::default_input_map())
        .add_systems(
            OnEnter(GameplayState::Playing),
            (spawn_crosshair, capture_cursor),
        )
        .add_systems(OnExit(GameplayState::Playing), release_cursor)
        // Purely aesthetic systems go in `Update`.
        .add_systems(
            Update,
            (
                capture_cursor.run_if(action_just_pressed(CursorAction::Capture)),
                release_cursor.run_if(action_just_pressed(CursorAction::Release)),
            )
                .run_if(in_state(GameplayState::Playing)),
        );
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CursorAction {
    Capture,
    Release,
}

impl CursorAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(CursorAction::Capture, MouseButton::Right);
        input_map.insert(CursorAction::Release, KeyCode::Escape);
        input_map
    }
}

/// Show a crosshair for better aiming
fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let crosshair_texture = asset_server.load("textures/crosshair.png");
    commands
        .spawn((
            StateScoped(GameplayState::Playing),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: crosshair_texture.into(),
                ..default()
            });
        });
}

fn capture_cursor(mut windows: Query<&mut Window>) {
    info!("capture_cursor");
    for mut window in &mut windows {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn release_cursor(mut windows: Query<&mut Window>) {
    info!("release_cursor");
    for mut window in &mut windows {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
