use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{
    Agent3d,
    prelude::{
        AgentDesiredVelocity3d as LandmassAgentDesiredVelocity, Velocity3d as LandmassVelocity, *,
    },
};
use bevy_tnua::prelude::*;

use crate::{demo::player::Player, screens::Screen};

use super::{NPC_FLOAT_HEIGHT, NPC_RADIUS, Npc};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        (setup_npc_agent, set_controller_velocity)
            .chain()
            .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        RunFixedMainLoop,
        sync_agent_velocity
            .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop)
            .run_if(in_state(Screen::Gameplay)),
    );
}

fn setup_npc_agent(
    mut commands: Commands,
    q_uninitialized: Query<Entity, (With<Npc>, Without<Agent3d>)>,
    player: Option<Single<Entity, With<Player>>>,
    archipelago: Option<Single<Entity, With<Archipelago3d>>>,
) {
    let Some(player) = player else {
        return;
    };
    let Some(archipelago) = archipelago else {
        return;
    };
    for entity in q_uninitialized.iter() {
        commands.entity(entity).insert((
            Agent3dBundle {
                agent: Default::default(),
                settings: AgentSettings {
                    radius: NPC_RADIUS,
                    desired_speed: 5.0,
                    max_speed: 8.0,
                },
                archipelago_ref: ArchipelagoRef3d::new(*archipelago),
            },
            AgentTarget3d::Entity(*player),
        ));
    }
}

/// Use the desired velocity as the agent's velocity.
fn set_controller_velocity(
    mut agent_query: Query<(&mut TnuaController, &LandmassAgentDesiredVelocity)>,
) {
    for (mut controller, desired_velocity) in agent_query.iter_mut() {
        let velocity = desired_velocity.velocity();
        let forward = if velocity.length_squared() > 0.1 {
            Dir3::try_from(velocity).ok()
        } else {
            None
        };
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: velocity,
            desired_forward: forward,
            float_height: NPC_FLOAT_HEIGHT,
            spring_strength: 1000.0,
            ..default()
        });
    }
}

fn sync_agent_velocity(mut agent_query: Query<(&LinearVelocity, &mut LandmassVelocity)>) {
    for (avian_velocity, mut landmass_velocity) in agent_query.iter_mut() {
        landmass_velocity.velocity = avian_velocity.0;
    }
}
