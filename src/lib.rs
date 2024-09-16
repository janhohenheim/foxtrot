mod animation;
mod asset_tracking;
mod audio;
mod character;
mod collision_layer;
mod cursor;
#[cfg(feature = "dev")]
mod dev_tools;
mod dialog;
mod hacks;
mod level;
mod player;
mod screens;
mod system_set;
mod theme;
mod ui_camera;

use avian3d::{prelude::SyncPlugin, sync::SyncConfig, PhysicsPlugins};
use avian_interpolation3d::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    log::LogPlugin,
    prelude::*,
};
use bevy_tweening::TweeningPlugin;
use blenvy::BlenvyPlugin;
use sickle_ui::SickleUiPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Foxtrot".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                })
                .set(LogPlugin {
                    // - Blenvy's alpha currently logs debug messages under the info level, so we disable that in general.
                    // - The ronstring_to_reflect_component is reporting a warning for `components_meta`, which is exported by mistake.
                    // - avian3d::prepare is reporting that dynamic rigid bodies are lacking mass, which is because it takes a while for the
                    //   underlying blueprint to finish loading.
                    filter: "\
                        blenvy=warn,\
                        blenvy::components::ronstring_to_reflect_component=error,\
                        avian3d::prepare=error\
                    "
                    .to_string(),
                    ..default()
                }),
        );

        // Add third party plugins.
        app.init_resource::<SyncConfig>();
        app.add_plugins((
            BlenvyPlugin::default(),
            PhysicsPlugins::default().build().disable::<SyncPlugin>(),
            AvianInterpolationPlugin::default(),
            SickleUiPlugin,
            TweeningPlugin,
        ));

        // Add internal plugins.
        app.add_plugins((
            asset_tracking::plugin,
            animation::plugin,
            player::plugin,
            level::plugin,
            screens::plugin,
            theme::plugin,
            collision_layer::plugin,
            ui_camera::plugin,
            character::plugin,
            system_set::plugin,
            hacks::plugin,
            cursor::plugin,
            dialog::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.add(ui_camera::spawn_ui_camera);
}
