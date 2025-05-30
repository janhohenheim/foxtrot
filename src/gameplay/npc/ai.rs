//! NPC AI. In this case, the only AI is the ability to move towards the player.

use std::f32::consts::TAU;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{
    TargetReachedCondition,
    prelude::{
        AgentDesiredVelocity3d as LandmassAgentDesiredVelocity, Velocity3d as LandmassVelocity, *,
    },
};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::prelude::*;

use crate::{
    PrePhysicsAppSystems, gameplay::player::navmesh_position::LastValidPlayerNavmeshPosition,
    screens::Screen,
};

use super::{NPC_FLOAT_HEIGHT, NPC_RADIUS, Npc};

pub(crate) const NPC_MAX_SLOPE: f32 = TAU / 6.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Agent>();
    app.register_type::<AgentOf>();
    app.register_type::<WantsToFollowPlayer>();
    app.add_systems(
        RunFixedMainLoop,
        (sync_agent_velocity, set_controller_velocity)
            .chain()
            .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop)
            .before(LandmassSystemSet::SyncExistence)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        RunFixedMainLoop,
        update_agent_target.in_set(PrePhysicsAppSystems::UpdateNavmeshTargets),
    );
    app.add_observer(setup_npc_agent);
}

/// Setup the NPC agent. An "agent" is what `bevy_landmass` can move around.
/// Since we use a floating character controller, we need to offset the agent's position by the character's float height.
#[cfg_attr(feature = "hot_patch", hot)]
fn setup_npc_agent(
    trigger: Trigger<OnAdd, Npc>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let npc = trigger.target();
    commands.spawn((
        Name::new("NPC Agent"),
        Transform::from_translation(Vec3::new(0.0, -NPC_FLOAT_HEIGHT, 0.0)),
        Agent3dBundle {
            agent: default(),
            settings: AgentSettings {
                radius: NPC_RADIUS,
                desired_speed: 7.0,
                max_speed: 8.0,
            },
            archipelago_ref: ArchipelagoRef3d::new(*archipelago),
        },
        TargetReachedCondition::Distance(Some(2.0)),
        ChildOf(npc),
        AgentOf(npc),
        AgentTarget3d::default(),
        WantsToFollowPlayer,
    ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct WantsToFollowPlayer;

#[cfg_attr(feature = "hot_patch", hot)]
fn update_agent_target(
    mut agents: Query<&mut AgentTarget3d, With<WantsToFollowPlayer>>,
    player_position: Single<&LastValidPlayerNavmeshPosition>,
) {
    let Some(player_position) = player_position.0 else {
        return;
    };
    for mut target in &mut agents {
        *target = AgentTarget3d::Point(player_position);
    }
}

#[derive(Component, Deref, Debug, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Agent)]
struct AgentOf(Entity);

#[derive(Component, Deref, Debug, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = AgentOf)]
struct Agent(Entity);

/// Use the desired velocity as the agent's velocity.
#[cfg_attr(feature = "hot_patch", hot)]
fn set_controller_velocity(
    mut agent_query: Query<(&mut TnuaController, &Agent)>,
    desired_velocity_query: Query<&LandmassAgentDesiredVelocity>,
) {
    for (mut controller, agent) in &mut agent_query {
        let Ok(desired_velocity) = desired_velocity_query.get(**agent) else {
            continue;
        };
        let velocity = desired_velocity.velocity();
        let forward = Dir3::try_from(velocity).ok();
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: velocity,
            desired_forward: forward,
            float_height: NPC_FLOAT_HEIGHT,
            spring_strength: 1500.0,
            max_slope: NPC_MAX_SLOPE,
            ..default()
        });
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn sync_agent_velocity(mut agent_query: Query<(&LinearVelocity, &mut LandmassVelocity)>) {
    for (avian_velocity, mut landmass_velocity) in &mut agent_query {
        landmass_velocity.velocity = avian_velocity.0;
    }
}
