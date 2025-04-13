//! Development tools for the game. This plugin is only enabled in dev builds.

use aalo::prelude::*;
use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_landmass::debug::{EnableLandmassDebug, Landmass3dDebugPlugin};
use input::{ForceFreeCursor, ToggleDebugUi};

mod input;

use crate::{gameplay::crosshair::cursor::IsCursorForcedFreed, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    app.add_plugins(input::plugin);

    app.init_resource::<DebugState>();

    app.add_plugins(AaloPlugin::new().world());

    app.add_plugins((
        DebugUiPlugin,
        PhysicsDebugPlugin::default(),
        Landmass3dDebugPlugin {
            draw_on_start: false,
            ..default()
        },
    ));
    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );
    app.add_observer(advance_debug_state);
    app.add_observer(toggle_cursor_forced_free);
    app.add_systems(
        Update,
        (
            toggle_debug_ui.run_if(toggled_state(DebugState::Ui)),
            toggle_physics_debug_ui.run_if(toggled_state(DebugState::Physics)),
            toggle_landmass_debug_ui.run_if(toggled_state(DebugState::Landmass)),
        )
            .chain(),
    );
}

fn advance_debug_state(
    _trigger: Trigger<Started<ToggleDebugUi>>,
    mut debug_state: ResMut<DebugState>,
) {
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

fn toggle_cursor_forced_free(
    _trigger: Trigger<Started<ForceFreeCursor>>,
    mut is_cursor_forced_free: ResMut<IsCursorForcedFreed>,
) {
    is_cursor_forced_free.0 = !is_cursor_forced_free.0;
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
