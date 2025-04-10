use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_landmass::{
    Agent3d,
    prelude::{
        AgentDesiredVelocity3d as LandmassAgentDesiredVelocity, Velocity3d as LandmassVelocity, *,
    },
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_trenchbroom::prelude::*;
use oxidized_navigation::NavMeshAffector;

use crate::{screens::Screen, third_party::bevy_trenchbroom::LoadTrenchbroomModel};

use super::player::Player;

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
            .chain()
            .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(Update, print_agent_state.run_if(in_state(Screen::Gameplay)));
}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/fox/Fox.gltf")]
#[component(on_add = Self::on_add)]
pub(crate) struct Npc;

impl Npc {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        if world.is_scene_world() {
            return;
        }
        let model = world
            .resource::<AssetServer>()
            .load_trenchbroom_model::<Self>();
        world.commands().entity(entity).insert((
            Npc,
            SceneRoot(model),
            TrenchBroomGltfRotationFix,
            TransformInterpolation,
            Collider::capsule(0.5, 0.5),
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ));
    }
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
                    radius: 0.5,
                    desired_speed: 2.0,
                    max_speed: 3.0,
                },
                archipelago_ref: ArchipelagoRef3d::new(*archipelago),
            },
            AgentTarget3d::Entity(*player),
        ));
    }
}

fn print_agent_state(mut agent_query: Query<&AgentState>) {
    for agent_state in agent_query.iter() {
        info!("Agent state: {:?}", agent_state);
    }
}

/// Use the desired velocity as the agent's velocity.
fn set_controller_velocity(
    mut agent_query: Query<(&mut TnuaController, &LandmassAgentDesiredVelocity)>,
) {
    for (mut controller, desired_velocity) in agent_query.iter_mut() {
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: desired_velocity.velocity() * 2.0,
            desired_forward: Dir3::try_from(desired_velocity.velocity()).ok(),
            float_height: 1.5,
            ..default()
        });
        info!("Set controller velocity to {}", desired_velocity.velocity());
    }
}

fn sync_agent_velocity(mut agent_query: Query<(&LinearVelocity, &mut LandmassVelocity)>) {
    for (avian_velocity, mut landmass_velocity) in agent_query.iter_mut() {
        landmass_velocity.velocity = avian_velocity.0;
        info!("Sync agent velocity to {}", landmass_velocity.velocity);
    }
}
