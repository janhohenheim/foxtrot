use super::bevy_trenchbroom::Worldspawn;
use avian3d::prelude::Collider;
use bevy::prelude::*;
use bevy_landmass::{PointSampleDistance3d, prelude::*};
use landmass_oxidized_navigation::{LandmassOxidizedNavigationPlugin, OxidizedArchipelago};
use oxidized_navigation::{ NavMeshAffector, NavMeshSettings, OxidizedNavigationPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Landmass3dPlugin::default(),
        LandmassOxidizedNavigationPlugin::default(),
        OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings {
            step_height: 5,
            ..NavMeshSettings::from_agent_and_bounds(0.5, 2.0, 100.0, -20.0)
        }),
    ));
    app.add_systems(Startup, setup_archipelago);
    app.add_observer(add_nav_mesh_affector_to_trenchbroom_worldspawn);
}

fn setup_archipelago(mut commands: Commands) {
    // This *should* be scoped to the `Screen::Gameplay` state, but doing so
    // seems to never regenerate the nav mesh when the level is loaded the second
    // time.
    commands.spawn((
        Name::new("Main Level Archipelago"),
        Archipelago3d::new(AgentOptions {
            point_sample_distance: PointSampleDistance3d {
                horizontal_distance: 0.25,
                distance_above: 1.0,
                distance_below: 2.0,
                vertical_preference_ratio: 2.0,
            },
            ..AgentOptions::from_agent_radius(0.6)
        }),
        OxidizedArchipelago,
    ));
}

fn add_nav_mesh_affector_to_trenchbroom_worldspawn(
    trigger: Trigger<OnAdd, Worldspawn>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).insert(NavMeshAffector);
}
