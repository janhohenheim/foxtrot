//! Toggles for the different debug UIs that our plugins provide.

use super::input::ToggleDebugUi;
//use aalo::prelude::*;
use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_landmass::debug::{EnableLandmassDebug, Landmass3dDebugPlugin};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();

    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: false,
            ..default()
        },
    });

    // TODO
    //app.add_plugins(AaloPlugin::new().world().flatten_descendants());

    app.add_plugins((
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
    // app.add_observer(toogle_aalo_inspector);
    // app.add_observer(disable_aalo_inspector_on_spawn);
    app.add_systems(
        Update,
        (
            toggle_fps_overlay.run_if(toggled_state(DebugState::None)),
            toggle_debug_ui.run_if(toggled_state(DebugState::Ui)),
            toggle_physics_debug_ui.run_if(toggled_state(DebugState::Physics)),
            toggle_landmass_debug_ui.run_if(toggled_state(DebugState::Landmass)),
        )
            .chain()
            .in_set(AppSet::ChangeUi),
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

fn toggle_fps_overlay(mut config: ResMut<FpsOverlayConfig>) {
    config.enabled = !config.enabled;
}

/*
fn disable_aalo_inspector_on_spawn(
    _trigger: Trigger<OnAdd, InspectorMarker>,
    mut aalo_inspector: Single<&mut Visibility, With<InspectorMarker>>,
) {
    **aalo_inspector = Visibility::Hidden;
}
*/

/*
fn toogle_aalo_inspector(
    _trigger: Trigger<Started<ForceFreeCursor>>,
    mut is_cursor_forced_free: ResMut<IsCursorForcedFreed>,
    mut aalo_inspector: Single<&mut Visibility, With<InspectorMarker>>,
) {
    is_cursor_forced_free.0 = !is_cursor_forced_free.0;
    **aalo_inspector = match **aalo_inspector {
        Visibility::Inherited => Visibility::Hidden,
        Visibility::Hidden => Visibility::Inherited,
        Visibility::Visible => Visibility::Inherited,
    };
}
*/

#[derive(Resource, Debug, Default, Eq, PartialEq)]
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
