use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

use crate::props::{effects::prepare_light_mesh, setup::static_bundle};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_lamp_wall_electric);
    app.register_type::<LampWallElectric>();
}

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model(
    "models/darkmod/lights/non-extinguishable/lamp_wall_electric_01/lamp_wall_electric_01.gltf"
)]
#[spawn_hook(preload_model::<Self>)]
#[classname("light_lamp_wall_electric")]
pub(crate) struct LampWallElectric;

fn setup_lamp_wall_electric(
    trigger: Trigger<OnAdd, LampWallElectric>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle =
        static_bundle::<LampWallElectric>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands
        .entity(trigger.target())
        .insert(bundle)
        .with_child((
            Transform::from_xyz(0.0, -0.08, -0.35),
            PointLight {
                color: Color::srgb(232.0 / 255.0, 199.0 / 255.0, 176.0 / 255.0),
                intensity: 20_000.0,
                radius: 0.05,
                range: 20.0,
                shadows_enabled: true,
                #[cfg(feature = "native")]
                soft_shadows_enabled: true,
                ..default()
            },
        ))
        .observe(prepare_light_mesh);
}
