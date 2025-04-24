//! Preload crosshair assets.

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(crate) const CROSSHAIR_DOT_PATH: &str = "ui/crosshair_dot.png";
pub(crate) const CROSSHAIR_SQUARE_PATH: &str = "ui/crosshair_square.png";

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CursorAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CursorAssets {
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
