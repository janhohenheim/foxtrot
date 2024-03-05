use crate::{
    file_system_interaction::asset_loading::GrassAssets, level_instantiation::on_spawn::Ground,
    GameState,
};
use bevy::{app::App, prelude::*, render::primitives::Aabb};
use warbler_grass::{
    bundle::{GrassColor, WarblerHeight, WarblersBundle},
    map::DensityMap,
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WarblersPlugin)
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

/// Spawns the grass using the ground as a base
fn spawn(
    mut commands: Commands,
    ground: Query<&Transform, Added<Ground>>,
    grass_assets: Res<GrassAssets>,
) {
    for transform in ground.iter() {
        let density_map = DensityMap::new(grass_assets.density_map.clone(), 5.);
        let offset = Vec3::new(transform.scale.x, 0., transform.scale.z);
        let aabb = Aabb::from_min_max(-offset, offset);
        let grass_transform =
            Transform::from_translation(-offset + transform.translation + Vec3::X);
        commands.spawn(WarblersBundle {
            density_map,
            grass_color: GrassColor {
                main_color: Color::rgb(0.3, 0.6, 0.0),
                bottom_color: Color::rgb(0.2, 0.1, 0.),
            },
            aabb,
            spatial: SpatialBundle::from_transform(grass_transform),
            height: WarblerHeight::Uniform(1.2),
            ..default()
        });
    }
}
