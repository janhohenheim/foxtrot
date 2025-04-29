//! [Landmass](https://github.com/andriyDev/landmass) powers out agent navigation.
//! The underlying navmesh is generated using [Oxidized Navigation](https://github.com/TheGrimsey/oxidized_navigation).

use super::bevy_trenchbroom::Worldspawn;
use crate::gameplay::npc::{NPC_HEIGHT, NPC_RADIUS, ai::NPC_MAX_SLOPE};
use avian3d::prelude::*;
use bevy::ecs::relationship::Relationship as _;
use bevy::prelude::*;
use bevy_landmass::{Character, PointSampleDistance3d, prelude::*};
use landmass_oxidized_navigation::{LandmassOxidizedNavigationPlugin, OxidizedArchipelago};
use oxidized_navigation::{
    NavMeshAffector, NavMeshSettings, OxidizedNavigationPlugin, colliders::avian::AvianCollider,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Landmass3dPlugin::default(),
        LandmassOxidizedNavigationPlugin::default(),
        OxidizedNavigationPlugin::<AvianCollider>::new(NavMeshSettings {
            step_height: 3,
            max_contour_simplification_error: 0.7,
            max_traversable_slope_radians: NPC_MAX_SLOPE,
            ..NavMeshSettings::from_agent_and_bounds(
                NPC_RADIUS * 0.95,
                NPC_HEIGHT * 0.95,
                100.0,
                -20.0,
            )
        }),
    ));
    app.add_systems(Startup, setup_archipelago);
    app.add_observer(add_nav_mesh_affector_to_trenchbroom_worldspawn);
    app.add_observer(add_nav_mesh_affector_to_colliders_under_nav_mesh_affector_parent);
}

fn setup_archipelago(mut commands: Commands) {
    // This *should* be scoped to the `Screen::Gameplay` state, but doing so
    // seems to never regenerate the nav mesh when the level is loaded the second
    // time.
    commands.spawn((
        Name::new("Main Level Archipelago"),
        Archipelago3d::new(AgentOptions {
            point_sample_distance: PointSampleDistance3d {
                horizontal_distance: 0.6,
                distance_above: 1.0,
                distance_below: 1.0,
                vertical_preference_ratio: 2.0,
            },
            ..AgentOptions::from_agent_radius(NPC_RADIUS)
        }),
        OxidizedArchipelago,
    ));
}

pub(crate) fn insert_landmass_character(
    In((entity, radius)): In<(Entity, f32)>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    commands.entity(entity).insert(Character3dBundle {
        character: Character::default(),
        settings: CharacterSettings { radius },
        archipelago_ref: ArchipelagoRef3d::new(*archipelago),
    });
}

#[derive(Component)]
pub(crate) struct NavMeshAffectorParent;

fn add_nav_mesh_affector_to_trenchbroom_worldspawn(
    trigger: Trigger<OnAdd, Worldspawn>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).insert(NavMeshAffector);
}

fn add_nav_mesh_affector_to_colliders_under_nav_mesh_affector_parent(
    trigger: Trigger<OnAdd, ColliderOf>,
    collider_parent: Query<&ColliderOf>,
    nav_mesh_affector_parent: Query<(), With<NavMeshAffectorParent>>,
    mut commands: Commands,
) {
    let collider = trigger.target();
    let rigid_body = collider_parent.get(collider).unwrap().get();
    if nav_mesh_affector_parent.contains(rigid_body) {
        commands.entity(collider).insert(NavMeshAffector);
    }
}
