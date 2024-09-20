use crate::{dialog::StartDialog, screens::gameplay::GameplayState};
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_yarnspinner::events::DialogueCompleteEvent;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CrosshairNode>()
        .add_plugins(InputManagerPlugin::<CursorAction>::default())
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
        )
        .add_systems(
            Update,
            capture_cursor_on_dialog_end.run_if(in_state(GameplayState::Playing)),
        )
        .observe(release_cursor_on_dialog_start)
        .observe(set_cursor_visibility);
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
            CrosshairNode,
            Name::new("Crosshair UI"),
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
                image: UiImage::from(crosshair_texture).with_color(Color::WHITE.with_alpha(0.4)),
                ..default()
            });
        });
}

fn capture_cursor(mut commands: Commands) {
    commands.trigger(SetCursorVisibility(false));
}

fn release_cursor(mut commands: Commands) {
    commands.trigger(SetCursorVisibility(true));
}

#[derive(Event, Debug, Copy, Clone, Deref, DerefMut)]
pub struct SetCursorVisibility(pub bool);

fn set_cursor_visibility(
    trigger: Trigger<SetCursorVisibility>,
    mut windows: Query<&mut Window>,
    mut crosshair_visibility: Query<&mut Visibility, With<CrosshairNode>>,
) {
    let &SetCursorVisibility(visible) = trigger.event();
    for mut window in &mut windows {
        window.cursor.visible = visible;
        window.cursor.grab_mode = if visible {
            CursorGrabMode::None
        } else {
            CursorGrabMode::Locked
        };
    }
    for mut crosshair in &mut crosshair_visibility {
        *crosshair = if visible {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct CrosshairNode;

fn release_cursor_on_dialog_start(_trigger: Trigger<StartDialog>, mut commands: Commands) {
    commands.trigger(SetCursorVisibility(true));
}

fn capture_cursor_on_dialog_end(
    mut reader: EventReader<DialogueCompleteEvent>,
    mut commands: Commands,
) {
    for _ in reader.read() {
        commands.trigger(SetCursorVisibility(false));
    }
}
