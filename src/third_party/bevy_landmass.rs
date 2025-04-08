use super::bevy_trenchbroom::Worldspawn;
use bevy::prelude::*;
use bevy_landmass::prelude::*;
use oxidized_navigation::NavMeshAffector;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Landmass3dPlugin::default());
    // TODO: add oxidized_navigation and landmass_oxidized_navigation plugins
    app.add_observer(add_nav_mesh_affector_to_trenchbroom_worldspawn);
}

fn add_nav_mesh_affector_to_trenchbroom_worldspawn(
    trigger: Trigger<OnAdd, Worldspawn>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).insert(NavMeshAffector);
}
