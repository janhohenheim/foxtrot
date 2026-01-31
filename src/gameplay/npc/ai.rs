//! NPC AI. In this case, the only AI is the ability to move towards the player.

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::input::GlobalMovement;
use bevy_enhanced_input::prelude::*;
use bevy_landmass::{
    TargetReachedCondition,
    prelude::{
        AgentDesiredVelocity3d as LandmassAgentDesiredVelocity, Velocity3d as LandmassVelocity, *,
    },
};

use crate::{
    gameplay::{npc::NPC_SPEED, player::navmesh_position::LastValidPlayerNavmeshPosition},
    screens::Screen,
};

use super::{NPC_FLOAT_HEIGHT, NPC_RADIUS, Npc};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            sync_agent_velocity,
            set_controller_velocity,
            rotate_npc,
            update_agent_target,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_observer(setup_npc_agent);
    app.add_input_context::<NpcInputContext>();
}

/// Setup the NPC agent. An "agent" is what `bevy_landmass` can move around.
/// Since we use a floating character controller, we need to offset the agent's position by the character's float height.
fn setup_npc_agent(
    add: On<Add, Npc>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let npc = add.entity;
    commands.entity(npc).insert((
        NpcInputContext,
        actions!(
            NpcInputContext[(
                Action::<GlobalMovement>::new(),
                ActionMock {
                    state: ActionState::None,
                    value: Vec3::ZERO.into(),
                    span: MockSpan::Updates(1),
                    enabled: false
                }
            )]
        ),
    ));
    commands.spawn((
        Name::new("NPC Agent"),
        Transform::from_translation(Vec3::new(0.0, -NPC_FLOAT_HEIGHT, 0.0)),
        Agent3dBundle {
            agent: default(),
            settings: AgentSettings {
                radius: NPC_RADIUS,
                desired_speed: NPC_SPEED,
                max_speed: NPC_SPEED + 1.0,
            },
            archipelago_ref: ArchipelagoRef3d::new(*archipelago),
        },
        TargetReachedCondition::Distance(Some(3.0)),
        ChildOf(npc),
        AgentOf(npc),
        AgentTarget3d::default(),
        WantsToFollowPlayer,
    ));
}

#[derive(Component)]
struct NpcInputContext;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct WantsToFollowPlayer;

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
fn set_controller_velocity(
    mut agent_query: Query<(&Agent, &Actions<NpcInputContext>)>,
    mut action_mocks: Query<&mut ActionMock, With<Action<GlobalMovement>>>,
    desired_velocity_query: Query<&LandmassAgentDesiredVelocity>,
) {
    for (agent, actions) in &mut agent_query {
        let Ok(desired_velocity) = desired_velocity_query.get(**agent) else {
            continue;
        };
        let velocity = desired_velocity.velocity();
        let mut iter = action_mocks.iter_many_mut(actions);
        let mut mock = iter.fetch_next().unwrap();

        if let Ok((dir, speed)) = Dir3::new_and_length(velocity) {
            let normalized = speed / NPC_SPEED;
            *mock = ActionMock::once(ActionState::Fired, dir * normalized);
        }
    }
}

fn rotate_npc(
    mut agent_query: Query<(&mut Transform, &LinearVelocity), With<Npc>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut agent_query {
        let hz_velocity = vec3(velocity.x, 0.0, velocity.z);
        if let Ok(dir) = Dir3::new(hz_velocity) {
            let target = transform.looking_to(dir, Vec3::Y).rotation;
            let decay_rate = f32::ln(600.0);
            transform
                .rotation
                .smooth_nudge(&target, decay_rate, time.delta_secs());
        }
    }
}

fn sync_agent_velocity(mut agent_query: Query<(&LinearVelocity, &mut LandmassVelocity)>) {
    for (avian_velocity, mut landmass_velocity) in &mut agent_query {
        landmass_velocity.velocity = avian_velocity.0;
    }
}
