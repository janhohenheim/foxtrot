//! Spawn the main level.

use bevy::prelude::*;
use bevy_trenchbroom::util::DoNotFixGltfRotationsUnderMe;

use crate::{asset_tracking::LoadResource, audio::music, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
}

/// A system that spawns the main level.
pub(crate) fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        SceneRoot(level_assets.level.clone()),
        StateScoped(Screen::Gameplay),
        Level,
        // We already fix the coordinate system for all glTFs in the app,
        // so we opt out of bevy_trenchbroom's coordinate system fixing.
        DoNotFixGltfRotationsUnderMe,
    ));

    commands.spawn((
        Name::new("Level Music"),
        music(level_assets.music.clone()),
        StateScoped(Screen::Gameplay),
    ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
    #[dependency]
    pub(crate) music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            // Our main level is inspired by the TheDarkMod fan mission [Volta I: The Stone](https://www.thedarkmod.com/missiondetails/?internalName=volta1_3)
            level: assets.load("maps/volta_i/volta_i.map#Scene"),
            music: assets.load("audio/music/Ambiance_Rain_Calm_Loop_Stereo.ogg"),
        }
    }
}
