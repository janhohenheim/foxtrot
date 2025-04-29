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
use bevy_tnua::prelude::*;

use crate::{gameplay::player::PlayerLandmassCharacter, screens::Screen};

use super::{NPC_FLOAT_HEIGHT, NPC_RADIUS, Npc};

pub(crate) const NPC_MAX_SLOPE: f32 = TAU / 8.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        (
            setup_npc_agent,
            sync_agent_velocity,
            set_controller_velocity,
        )
            .chain()
            .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop)
            .before(LandmassSystemSet::SyncExistence)
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// Setup the NPC agent. An "agent" is what `bevy_landmass` can move around.
/// Since we use a floating character controller, we need to offset the agent's position by the character's float height.
fn setup_npc_agent(
    mut commands: Commands,
    q_uninitialized: Query<Entity, (With<Npc>, Without<NpcAgent>)>,
    player: Single<&PlayerLandmassCharacter>,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    for entity in q_uninitialized.iter() {
        let agent = commands
            .spawn((
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
                AgentTarget3d::Entity(player.0),
                TargetReachedCondition::Distance(Some(2.0)),
                ChildOf(entity),
            ))
            .id();
        commands.entity(entity).insert(NpcAgent(agent));
    }
}

/// A link from the NPC itself to its agent entity.
#[derive(Component)]
struct NpcAgent(Entity);

/// Use the desired velocity as the agent's velocity.
fn set_controller_velocity(
    mut agent_query: Query<(&mut TnuaController, &NpcAgent)>,
    desired_velocity_query: Query<&LandmassAgentDesiredVelocity>,
) {
    for (mut controller, npc_agent) in &mut agent_query {
        let Ok(desired_velocity) = desired_velocity_query.get(npc_agent.0) else {
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

fn sync_agent_velocity(mut agent_query: Query<(&LinearVelocity, &mut LandmassVelocity)>) {
    for (avian_velocity, mut landmass_velocity) in &mut agent_query {
        landmass_velocity.velocity = avian_velocity.0;
    }
}
