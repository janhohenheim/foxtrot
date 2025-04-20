//! Preload crosshair assets.

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CursorAssets>();
    app.load_resource::<CursorAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct CursorAssets {
    #[dependency]
    pub(crate) crosshair_dot: Handle<Image>,
    pub(crate) crosshair_square: Handle<Image>,
}

impl FromWorld for CursorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            crosshair_dot: assets.load("ui/crosshair_dot.png"),
            crosshair_square: assets.load("ui/crosshair_square.png"),
        }
    }
}
