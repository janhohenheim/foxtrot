//! Toggles for the different debug UIs that our plugins provide.

use std::any::Any as _;

use super::input::{ForceFreeCursor, ToggleDebugUi};
use crate::RenderLayer;
use crate::gameplay::crosshair::CrosshairState;
use crate::{PostPhysicsAppSystems, theme::widget};
use avian3d::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::Val::*;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_landmass::debug::{EnableLandmassDebug, Landmass3dDebugPlugin, LandmassGizmoConfigGroup};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();
    app.init_resource::<InspectorActive>();
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: false,
            ..default()
        },
    });
    app.add_plugins((
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        WorldInspectorPlugin::new().run_if(is_inspector_active),
    ));

    app.add_plugins((
        PhysicsDebugPlugin::default(),
        PhysicsDiagnosticsPlugin,
        PhysicsDiagnosticsUiPlugin,
        Landmass3dDebugPlugin {
            draw_on_start: false,
            ..default()
        },
    ));
    app.insert_resource(PhysicsDiagnosticsUiSettings {
        enabled: false,
        ..default()
    });
    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );
    app.insert_gizmo_config(
        LightGizmoConfigGroup {
            draw_all: true,
            ..default()
        },
        GizmoConfig {
            enabled: false,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );
    app.insert_gizmo_config(
        LandmassGizmoConfigGroup,
        GizmoConfig {
            enabled: true,
            depth_bias: -0.03,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );
    app.add_observer(advance_debug_state);
    app.add_observer(toggle_egui_inspector);
    app.add_systems(Startup, setup_debug_ui_text);
    app.add_systems(
        Update,
        update_debug_ui_text.run_if(resource_exists_and_changed::<DebugState>),
    );
    app.add_systems(
        Update,
        (
            toggle_fps_overlay.run_if(toggled_state(DebugState::None)),
            toggle_debug_ui.run_if(toggled_state(DebugState::Ui)),
            toggle_lighting_debug_ui.run_if(toggled_state(DebugState::Lighting)),
            toggle_physics_debug_ui.run_if(toggled_state(DebugState::Physics)),
            toggle_landmass_debug_ui.run_if(toggled_state(DebugState::Landmass)),
        )
            .chain()
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_debug_ui_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Debug UI"),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            justify_content: JustifyContent::End,
            ..default()
        },
        Pickable::IGNORE,
        children![(widget::label("Debug UI"), DebugUiText)],
    ));
}

#[derive(Component)]
struct DebugUiText;

#[cfg_attr(feature = "hot_patch", hot)]
fn advance_debug_state(
    _trigger: Trigger<Started<ToggleDebugUi>>,
    mut debug_state: ResMut<DebugState>,
) {
    *debug_state = debug_state.next();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn update_debug_ui_text(
    debug_state: Res<DebugState>,
    mut text: Single<&mut Text, With<DebugUiText>>,
) {
    text.0 = match *debug_state {
        DebugState::None => "",
        DebugState::Ui => "Ui",
        DebugState::Lighting => "Lighting",
        DebugState::Physics => "Physics",
        DebugState::Landmass => "Landmass",
    }
    .to_string();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn toggle_physics_debug_ui(
    mut config_store: ResMut<GizmoConfigStore>,
    mut physics_diagnostics: ResMut<PhysicsDiagnosticsUiSettings>,
) {
    let config = config_store.config_mut::<PhysicsGizmos>().0;
    config.enabled = !config.enabled;
    physics_diagnostics.enabled = !physics_diagnostics.enabled;
}

#[cfg_attr(feature = "hot_patch", hot)]
fn toggle_lighting_debug_ui(mut config_store: ResMut<GizmoConfigStore>) {
    let config = config_store.config_mut::<LightGizmoConfigGroup>().0;
    config.enabled = !config.enabled;
}

#[cfg_attr(feature = "hot_patch", hot)]
fn toggle_landmass_debug_ui(mut debug: ResMut<EnableLandmassDebug>) {
    **debug = !**debug;
}

#[cfg_attr(feature = "hot_patch", hot)]
fn toggle_fps_overlay(mut config: ResMut<FpsOverlayConfig>) {
    config.enabled = !config.enabled;
}

#[cfg_attr(feature = "hot_patch", hot)]
fn toggle_egui_inspector(
    _trigger: Trigger<Started<ForceFreeCursor>>,
    mut crosshair_state: Single<&mut CrosshairState>,
    mut inspector_active: ResMut<InspectorActive>,
) {
    inspector_active.0 = !inspector_active.0;
    if inspector_active.0 {
        crosshair_state
            .wants_free_cursor
            .insert(toggle_egui_inspector.type_id());
    } else {
        crosshair_state
            .wants_free_cursor
            .remove(&toggle_egui_inspector.type_id());
    }
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
struct InspectorActive(bool);

fn is_inspector_active(inspector_active: Res<InspectorActive>) -> bool {
    inspector_active.0
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
enum DebugState {
    #[default]
    None,
    Ui,
    Lighting,
    Physics,
    Landmass,
}

impl DebugState {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::Ui,
            Self::Ui => Self::Lighting,
            Self::Lighting => Self::Physics,
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
