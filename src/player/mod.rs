//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{
    animation::PlayerAnimation,
    asset_tracking::LoadResource,
    movement::{MovementController, ScreenWrap},
    screens::Screen,
    AppSet,
};

pub mod camera;
pub mod input;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );

    app.add_plugins((camera::plugin, input::plugin));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// A command to spawn the player character.
#[derive(Debug, Default)]
pub struct SpawnPlayer;

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    // This #[dependency] attribute marks the field as a dependency of the Asset.
    // This means that it will not finish loading until the labeled asset is also loaded.
    #[dependency]
    pub ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl PlayerAssets {
    pub const PATH_DUCKY: &'static str = "images/ducky.png";
    pub const PATH_STEP_1: &'static str = "audio/sound_effects/step1.ogg";
    pub const PATH_STEP_2: &'static str = "audio/sound_effects/step2.ogg";
    pub const PATH_STEP_3: &'static str = "audio/sound_effects/step3.ogg";
    pub const PATH_STEP_4: &'static str = "audio/sound_effects/step4.ogg";
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                PlayerAssets::PATH_DUCKY,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load(PlayerAssets::PATH_STEP_1),
                assets.load(PlayerAssets::PATH_STEP_2),
                assets.load(PlayerAssets::PATH_STEP_3),
                assets.load(PlayerAssets::PATH_STEP_4),
            ],
        }
    }
}
