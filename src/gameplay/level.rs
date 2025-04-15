//! Spawn the main level.

use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{
    props::{load_model, *},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LevelAssets>();
    app.register_type::<Level>();
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(world: &mut World) {
    let assets = world.resource::<LevelAssets>();
    world
        .spawn((
            Name::new("Level"),
            SceneRoot(assets.level.clone()),
            StateScoped(Screen::Gameplay),
            Level,
        ))
        .observe(advance_to_gameplay_screen);
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

fn advance_to_gameplay_screen(
    _trigger: Trigger<SceneInstanceReady>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Gameplay);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
    #[dependency]
    pub(crate) props: Vec<Handle<Scene>>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            //  Run ./scripts/compile_maps.sh and change .map to .bsp when we're done prototyping and want some extra performance
            level: assets.load("maps/foxtrot/foxtrot.map#Scene"),
            // We preload all props used in the level here. If we miss one, we don't get an error or anything like that,
            // but it will load during gameplay, which may cause a hiccup.
            props: vec![
                load_model::<Book>(assets),
                load_model::<Candle>(assets),
                load_model::<CandleUnlit>(assets),
                load_model::<Mug>(assets),
                load_model::<Plate>(assets),
            ],
        }
    }
}
