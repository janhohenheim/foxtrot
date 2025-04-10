use crate::screens::Screen;

use super::bevy_trenchbroom::Worldspawn;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::prelude::*;
use landmass_oxidized_navigation::{LandmassOxidizedNavigationPlugin, OxidizedArchipelago};
use oxidized_navigation::{NavMeshAffector, NavMeshSettings, OxidizedNavigationPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Landmass3dPlugin::default(),
        LandmassOxidizedNavigationPlugin,
        OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings {
            step_height: 6,
            ..NavMeshSettings::from_agent_and_bounds(0.5, 2.0, 100.0, -20.0)
        }),
    ));
    app.add_systems(OnEnter(Screen::Gameplay), setup_archipelago);
    app.add_observer(add_nav_mesh_affector_to_trenchbroom_worldspawn);
}

fn setup_archipelago(mut commands: Commands) {
    let archipelago_entity = commands
        .spawn((
            Name::new("Archipelago"),
            Archipelago3d::new(AgentOptions::default_for_agent_radius(0.5)),
            OxidizedArchipelago,
            StateScoped(Screen::Gameplay),
        ))
        .id();
}

fn add_nav_mesh_affector_to_trenchbroom_worldspawn(
    trigger: Trigger<OnAdd, Worldspawn>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).insert(NavMeshAffector);
}
