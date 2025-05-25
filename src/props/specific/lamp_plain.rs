use avian3d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_trenchbroom::prelude::*;

use crate::props::{effects::disable_shadow_casting_on_instance_ready, setup::static_bundle};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_lamp_wall_electric);
    app.register_type::<LampPlain>();
}

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/lights/non-extinguishable/electric_plain1_unattached.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
#[classname("light_lamp_plain")]
struct LampPlain {
    color: Color,
    intensity: f32,
}

impl Default for LampPlain {
    fn default() -> Self {
        Self {
            color: Color::srgb_u8(180, 180, 232),
            intensity: 13_000.0,
        }
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_lamp_wall_electric(
    trigger: Trigger<OnAdd, LampPlain>,
    lamp: Query<&LampPlain>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let lamp = lamp.get(trigger.target()).unwrap();
    let bundle = static_bundle::<LampPlain>(&asset_server, ColliderConstructor::ConvexHullFromMesh);
    commands
        .entity(trigger.target())
        .insert(bundle)
        .with_child((
            Transform::from_xyz(0.0, -0.08, -0.35),
            PointLight {
                color: lamp.color,
                intensity: lamp.intensity,
                radius: 0.05,
                range: 20.0,
                shadows_enabled: true,
                #[cfg(feature = "native")]
                soft_shadows_enabled: true,
                ..default()
            },
        ))
        .observe(disable_shadow_casting_on_instance_ready);
}
