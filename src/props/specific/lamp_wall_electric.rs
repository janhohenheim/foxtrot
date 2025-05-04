use avian3d::prelude::*;
use bevy::prelude::*;

use crate::props::{LampWallElectric, effects::prepare_light_mesh, generic::static_bundle};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_lamp_wall_electric);
}

pub(crate) fn setup_lamp_wall_electric(
    trigger: Trigger<OnAdd, LampWallElectric>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle =
        static_bundle::<LampWallElectric>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands
        .entity(trigger.target())
        // The prop should be held upright.
        .insert(bundle)
        // The lamp's origin is at the bottom of the lamp, so we need to offset the light a bit.
        .with_child((
            Transform::from_xyz(0.0, 0.2, 0.0),
            PointLight {
                color: Color::srgb(1.0, 0.7, 0.4),
                intensity: 40_000.0,
                radius: 0.2,
                shadows_enabled: true,
                #[cfg(feature = "native")]
                soft_shadows_enabled: true,

                ..default()
            },
        ))
        .observe(prepare_light_mesh);
}
