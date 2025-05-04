use avian_pickup::prop::PreferredPickupRotation;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::props::{LampSitting, effects::prepare_light_mesh, generic::dynamic_bundle};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_lamp_sitting);
}

pub(crate) fn setup_lamp_sitting(
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
                radius: 0.2,
                shadows_enabled: true,
                ..default()
            },
        ))
        .observe(prepare_light_mesh);
}
