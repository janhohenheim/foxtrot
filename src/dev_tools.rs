//! Development tools for the game. This plugin is only enabled in dev builds.

use aalo::prelude::*;
use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_landmass::debug::{EnableLandmassDebug, Landmass3dDebugPlugin};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    app.init_resource::<DebugState>();

    // Toggle the debug overlay for UI.
    app.add_plugins((
        DebugUiPlugin,
        PhysicsDebugPlugin::default(),
        Landmass3dDebugPlugin {
            draw_on_start: false,
            ..default()
        },
        AaloPlugin::new().world(),
    ));
    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );
    app.add_systems(
        Update,
        (
            advance_debug_state.run_if(input_just_pressed(TOGGLE_KEY)),
            toggle_debug_ui.run_if(toggled_state(DebugState::Ui)),
            toggle_physics_debug_ui.run_if(toggled_state(DebugState::Physics)),
            toggle_landmass_debug_ui.run_if(toggled_state(DebugState::Landmass)),
        )
            .chain(),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn advance_debug_state(mut debug_state: ResMut<DebugState>) {
    *debug_state = debug_state.next();
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_physics_debug_ui(mut config_store: ResMut<GizmoConfigStore>) {
    let config = config_store.config_mut::<PhysicsGizmos>().0;
    config.enabled = !config.enabled;
}

fn toggle_landmass_debug_ui(mut debug: ResMut<EnableLandmassDebug>) {
    **debug = !**debug;
}

#[derive(Debug, Resource, Default, Eq, PartialEq)]
enum DebugState {
    #[default]
    None,
    Ui,
    Physics,
    Landmass,
}
impl DebugState {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::Ui,
            Self::Ui => Self::Physics,
            Self::Physics => Self::Landmass,
            Self::Landmass => Self::None,
        }
    }
}

fn toggled_state(state: DebugState) -> impl Condition<()> {
    IntoSystem::into_system(move |current_state: Res<DebugState>| {
        let was_just_changed = current_state.is_changed() && !current_state.is_added();
        let entered_state = *current_state == state;
        let exited_state = *current_state == state.next();
        was_just_changed && (entered_state || exited_state)
    })
}
