//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

mod compile_shaders;
mod preload_assets;
mod spawn_level;

use crate::{asset_tracking::ResourceHandles, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<LoadingScreen>();
}

/// The game's main screen states.
#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Screen = Screen::Loading)]
#[states(scoped_entities)]
pub(crate) enum LoadingScreen {
    #[default]
    Assets,
    Shaders,
    Level,
}

pub(crate) trait LoadLevelExt {
    fn load_level(&mut self, level: Handle<Scene>);
}
impl LoadLevelExt for Commands<'_, '_> {
    fn load_level(&mut self, level: Handle<Scene>) {
        self.queue(LoadLevel(level));
    }
}

pub(crate) struct LoadLevel(Handle<Scene>);
impl Command for LoadLevel {
    fn apply(self, world: &mut World) {
        if !world.resource::<ResourceHandles>().is_all_done() {
            world
                .resource_mut::<NextState<LoadingScreen>>()
                .set(LoadingScreen::Assets);
        } else {
            world
                .resource_mut::<NextState<LoadingScreen>>()
                .set(LoadingScreen::Level);
        }
    }
}
