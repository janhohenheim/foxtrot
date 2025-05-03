//! Preload crosshair assets.

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(crate) const CROSSHAIR_DOT_PATH: &str = {
    #[cfg(feature = "dev")]
    {
        "ui/crosshair_dot.png"
    }
    #[cfg(not(feature = "dev"))]
    {
        "ui/crosshair_dot.ktx2"
    }
};

pub(crate) const CROSSHAIR_SQUARE_PATH: &str = {
    #[cfg(feature = "dev")]
    {
        "ui/crosshair_square.png"
    }
    #[cfg(not(feature = "dev"))]
    {
        "ui/crosshair_square.ktx2"
    }
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CursorAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct CursorAssets {
    #[dependency]
    crosshair_dot: Handle<Image>,
    #[dependency]
    crosshair_square: Handle<Image>,
}

impl FromWorld for CursorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            crosshair_dot: assets.load(CROSSHAIR_DOT_PATH),
            crosshair_square: assets.load(CROSSHAIR_SQUARE_PATH),
        }
    }
}
