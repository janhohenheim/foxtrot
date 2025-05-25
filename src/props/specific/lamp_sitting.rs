use avian_pickup::prop::PreferredPickupRotation;
use avian3d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_trenchbroom::prelude::*;

use crate::props::{effects::disable_shadow_casting_on_instance_ready, setup::dynamic_bundle};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_lamp_sitting);
    app.register_type::<LampSitting>();
}

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model(
    "models/darkmod/lights/non-extinguishable/round_lantern_sitting/round_lantern_sitting.gltf"
)]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct LampSitting;

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_lamp_sitting(
    trigger: Trigger<OnAdd, LampSitting>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let bundle = dynamic_bundle::<LampSitting>(
        &asset_server,
        ColliderConstructor::ConvexDecompositionFromMesh,
    );
    commands
        .entity(trigger.target())
        // The prop should be held upright.
        .insert((bundle, PreferredPickupRotation(Quat::IDENTITY)))
        // The lamp's origin is at the bottom of the lamp, so we need to offset the light a bit.
        .with_child((
            Transform::from_xyz(0.0, 0.2, 0.0),
            PointLight {
                color: Color::srgb(1.0, 0.7, 0.4),
                intensity: 40_000.0,
                radius: 0.05,
                shadows_enabled: true,
                #[cfg(feature = "native")]
                soft_shadows_enabled: true,
                ..default()
            },
        ))
        .observe(disable_shadow_casting_on_instance_ready);
}
