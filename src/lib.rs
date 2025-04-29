mod asset_tracking;
mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod gameplay;
mod props;
mod rendering;
mod screens;
mod theme;
mod third_party;
mod ui_camera;

use bitflags::bitflags;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
    prelude::*,
    render::view::RenderLayers,
};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppSet` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSet::TickTimers,
                AppSet::ChangeUi,
                AppSet::PlaySounds,
                AppSet::PlayAnimations,
                AppSet::Update,
            )
                .chain(),
        );

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
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::Linear(0.3),
                    },
                    ..default()
                }),
        );
        #[cfg(not(target_family = "wasm"))]
        app.add_plugins(TemporalAntiAliasPlugin);

        // Add third-party plugins.
        app.add_plugins(third_party::plugin);

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            gameplay::plugin,
            props::plugin,
            screens::plugin,
            theme::plugin,
            ui_camera::plugin,
            rendering::plugin,
        ));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Change UI.
    ChangeUi,
    /// Play sounds.
    PlaySounds,
    /// Play animations.
    PlayAnimations,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// This enum is converted to an `isize` to be used as a camera's order.
/// Since we have three camera, we use three enum variants.
/// This ordering here mean UI > ViewModel > World.
enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}

bitflags! {
    struct RenderLayer: u32 {
        /// Used implicitly by all entities without a `RenderLayers` component.
        /// Our world model camera and all objects other than the player are on this layer.
        /// The light source belongs to both layers.
        const DEFAULT = 0b00000001;
        /// Used by the view model camera and the player's arm.
        /// The light source belongs to both layers.
        const VIEW_MODEL = 0b00000010;
        /// Since we use multiple cameras, we need to be explicit about
        /// which one is allowed to render particles.
        const PARTICLES = 0b00000100;
        /// Skip interaction with lights
        const TRANSLUCENT = 0b00001000;
    }
}

impl From<RenderLayer> for RenderLayers {
    fn from(layer: RenderLayer) -> Self {
        // Bevy's default render layer is 0, so we need to subtract 1 from our bitfalgs to get the correct value.
        RenderLayers::from_iter(layer.iter().map(|l| l.bits() as usize - 1))
    }
}
